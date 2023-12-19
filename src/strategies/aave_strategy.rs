use super::types::Config;
use crate::collectors::time_collector::NewTick;
use anyhow::{anyhow, Result};
use artemis_core::executors::mempool_executor::{GasBidInfo, SubmitTxToMempool};
use artemis_core::types::Strategy;
use async_trait::async_trait;
use bindings_aave::{
    i_aave_oracle::IAaveOracle,
    i_pool_data_provider::IPoolDataProvider,
    ierc20::IERC20,
    l2_encoder::L2Encoder,
    pool::{BorrowFilter, Pool, SupplyFilter},
};
use bindings_liquidator::{
    liquidator::Liquidator,
    liquidator_paraswap::{LiquidationParams, LiquidatorParaswap, SwapParams},
};
use clap::{Parser, ValueEnum};
use ethers::{
    abi::{encode_packed, Token},
    contract::builders::ContractCall,
    providers::Middleware,
    types::{
        transaction::eip2718::TypedTransaction, Address, Bytes, ValueOrArray, H160, I256, U256, U64,
    },
};
use ethers_contract::Multicall;
use futures::join;
use paraswap_api::{
    apis::{configuration::Configuration, prices_api, transactions_api},
    models::TransactionsRequestPayload,
};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Write;
use std::iter::zip;
use std::str::FromStr;
use std::sync::Arc;
use tracing::{debug, error, info};

use super::types::{Action, Event};

#[derive(Debug)]
struct DeploymentConfig {
    pool_address: Address,
    pool_data_provider: Address,
    oracle_address: Address,
    l2_encoder: Address,
    creation_block: u64,
}

#[derive(Debug, Clone, Default, Parser, ValueEnum)]
pub enum Deployment {
    AAVE,
    #[default]
    SEASHELL,
}

#[derive(Debug, Clone, Default, Parser, ValueEnum, PartialEq)]
pub enum DexAggregator {
    #[default]
    None,
    Paraswap,
}

pub const WETH_ADDRESS: &str = "0x4200000000000000000000000000000000000006";
pub const MULTICALL3_ADDRESS: &str = "0xcA11bde05977b3631167028862bE2a173976CA11";

pub const LIQUIDATION_CLOSE_FACTOR_THRESHOLD: u64 = 950000000000000000;
pub const MAX_LIQUIDATION_CLOSE_FACTOR: u64 = 10000;
pub const DEFAULT_LIQUIDATION_CLOSE_FACTOR: u64 = 5000;

// admin stuff
pub const LOG_BLOCK_RANGE: u64 = 1000;
pub const MULTICALL_CHUNK_SIZE: usize = 100;
pub const STATE_CACHE_FILE: &str = "borrowers.json";
pub const PRICE_ONE: u64 = 100000000;
pub const PERCENT_HUNDRED: u64 = 10000;

fn get_deployment_config(deployment: Deployment) -> DeploymentConfig {
    match deployment {
        Deployment::AAVE => DeploymentConfig {
            pool_address: Address::from_str("0xA238Dd80C259a72e81d7e4664a9801593F98d1c5").unwrap(),
            pool_data_provider: Address::from_str("0x2d8A3C5677189723C4cB8873CfC9C8976FDF38Ac")
                .unwrap(),
            oracle_address: Address::from_str("0x2Cc0Fc26eD4563A5ce5e8bdcfe1A2878676Ae156")
                .unwrap(),
            l2_encoder: Address::from_str("0x39e97c588B2907Fb67F44fea256Ae3BA064207C5").unwrap(),
            creation_block: 2963358,
        },
        Deployment::SEASHELL => DeploymentConfig {
            pool_address: Address::from_str("0x8F44Fd754285aa6A2b8B9B97739B79746e0475a7").unwrap(),
            pool_data_provider: Address::from_str("0x2A0979257105834789bC6b9E1B00446DFbA8dFBa")
                .unwrap(),
            oracle_address: Address::from_str("0xFDd4e83890BCcd1fbF9b10d71a5cc0a738753b01")
                .unwrap(),
            l2_encoder: Address::from_str("0xceceF475167f7BFD8995c0cbB577644b623cD7Cf").unwrap(),
            creation_block: 3318602,
        },
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StateCache {
    last_block_number: u64,
    borrowers: HashMap<Address, Borrower>,
}

struct PoolState {
    prices: HashMap<Address, U256>,
    flashloan_premium_total: U256,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Borrower {
    address: Address,
    collateral: HashSet<Address>,
    debt: HashSet<Address>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenConfig {
    address: Address,
    a_address: Address,
    decimals: u64,
    ltv: u64,
    liquidation_threshold: u64,
    liquidation_bonus: u64,
    reserve_factor: u64,
    protocol_fee: u64,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct AaveStrategy<M> {
    /// Ethers client.
    client: Arc<M>,
    /// Amount of profits to bid in gas
    bid_percentage: u64,
    last_block_number: u64,
    borrowers: HashMap<Address, Borrower>,
    tokens: HashMap<Address, TokenConfig>,
    chain_id: u64,
    config: DeploymentConfig,
    liquidator: Address,
    dex_aggregator: DexAggregator,
    paraswap_api_config: Configuration,
}

impl<M: Middleware + 'static> AaveStrategy<M> {
    pub fn new(
        client: Arc<M>,
        config: Config,
        deployment: Deployment,
        liquidator_address: String,
        dex_aggregator: DexAggregator,
    ) -> Self {
        Self {
            client,
            bid_percentage: config.bid_percentage,
            last_block_number: 0,
            borrowers: HashMap::new(),
            tokens: HashMap::new(),
            chain_id: config.chain_id,
            config: get_deployment_config(deployment),
            liquidator: Address::from_str(&liquidator_address).expect("invalid liquidator address"),
            dex_aggregator,
            paraswap_api_config: Configuration::new(),
        }
    }
}

struct LiquidationOpportunity {
    borrower: Address,
    collateral: Address,
    debt: Address,
    debt_to_cover: U256,
    collateral_to_liquidate: U256,
    profit_eth: U256,
}

#[async_trait]
impl<M: Middleware + 'static> Strategy<Event, Action> for AaveStrategy<M> {
    // In order to sync this strategy, we need to get the current bid for all Sudo pools.
    async fn sync_state(&mut self) -> Result<()> {
        info!("syncing state");

        self.update_token_configs().await?;

        if self.dex_aggregator == DexAggregator::None {
            self.approve_tokens().await?;
        }

        self.load_cache()?;
        self.update_state().await?;

        info!("done syncing state");
        Ok(())
    }

    // Process incoming events, seeing if we can arb new orders, and updating the internal state on new blocks.
    async fn process_event(&mut self, event: Event) -> Option<Action> {
        match event {
            // Event::NewBlock(block) => self.process_new_block_event(block).await,
            Event::NewTick(block) => self.process_new_tick_event(block).await,
        }
    }
}

impl<M: Middleware + 'static> AaveStrategy<M> {
    /// Process new block events, updating the internal state.
    // async fn process_new_block_event(&mut self, event: NewBlock) -> Option<Action> {
    //     info!("received new block: {:?}", event);
    //     self.last_block_number = event.number.as_u64();
    //     None
    // }

    /// Process new block events, updating the internal state.
    async fn process_new_tick_event(&mut self, event: NewTick) -> Option<Action> {
        info!("received new tick: {:?}", event);
        self.update_token_configs()
            .await
            .map_err(|e| error!("Update token configs error: {}", e))
            .ok()?;

        self.update_state()
            .await
            .map_err(|e| error!("Update State error: {}", e))
            .ok()?;

        info!("Total borrower count: {}", self.borrowers.len());
        let op = self
            .get_best_liquidation_op()
            .await
            .map_err(|e| error!("Error finding liq ops: {}", e))
            .ok()??;

        info!("Best op - profit: {}", op.profit_eth);

        if op.profit_eth <= U256::from(0) {
            info!("No profitable ops, passing");
            return None;
        }

        return Some(Action::SubmitTx(SubmitTxToMempool {
            tx: self
                .build_liquidation(&op)
                .await
                .map_err(|e| error!("Error building liquidation: {}", e))
                .ok()?,
            gas_bid_info: Some(GasBidInfo {
                bid_percentage: self.bid_percentage,
                total_profit: U256::from_dec_str(&op.profit_eth.to_string())
                    .map_err(|e| error!("Failed to bid: {}", e))
                    .ok()?,
            }),
        }));
    }

    // for all known borrowers, return a sorted set of those with health factor < 1
    async fn get_underwater_borrowers(&mut self) -> Result<Vec<(Address, U256)>> {
        let pool = Pool::<M>::new(self.config.pool_address, self.client.clone());

        let mut underwater_borrowers = Vec::new();

        // call pool.getUserAccountData(user) for each borrower
        let mut multicall = Multicall::new(
            self.client.clone(),
            Some(H160::from_str(MULTICALL3_ADDRESS)?),
        )
        .await?;
        let borrowers: Vec<&Borrower> = self
            .borrowers
            .values()
            .filter(|b| b.debt.len() > 0)
            .collect();

        for chunk in borrowers.chunks(MULTICALL_CHUNK_SIZE) {
            multicall.clear_calls();

            for borrower in chunk {
                multicall.add_call(pool.get_user_account_data(borrower.address), false);
            }

            let result: Vec<(U256, U256, U256, U256, U256, U256)> = multicall.call_array().await?;
            for (borrower, (_, _, _, _, _, health_factor)) in zip(chunk, result) {
                if health_factor.lt(&U256::from_dec_str("1000000000000000000").unwrap()) {
                    info!(
                        "Found underwater borrower {:?} -  healthFactor: {}",
                        borrower, health_factor
                    );
                    underwater_borrowers.push((borrower.address, health_factor));
                }
            }
        }

        // sort borrowers by health factor
        underwater_borrowers.sort_by(|a, b| a.1.cmp(&b.1));
        Ok(underwater_borrowers)
    }

    // load borrower state cache from file if exists
    fn load_cache(&mut self) -> Result<()> {
        match File::open(STATE_CACHE_FILE) {
            Ok(file) => {
                let cache: StateCache = serde_json::from_reader(file)?;
                info!("read state cache from file");
                self.last_block_number = cache.last_block_number;
                self.borrowers = cache.borrowers;
            }
            Err(_) => {
                info!("no state cache file found, creating new one");
                self.last_block_number = self.config.creation_block;
            }
        };

        Ok(())
    }

    // update known borrower state from last block to latest block
    async fn update_state(&mut self) -> Result<()> {
        let latest_block = self.client.get_block_number().await?;
        info!(
            "Updating state from block {} to {}",
            self.last_block_number, latest_block
        );

        self.get_borrow_logs(self.last_block_number.into(), latest_block)
            .await?
            .into_iter()
            .for_each(|log| {
                let user = log.on_behalf_of;
                // fetch assets if user already a borrower
                if self.borrowers.contains_key(&user) {
                    let borrower = self.borrowers.get_mut(&user).unwrap();
                    borrower.debt.insert(log.reserve);
                    return;
                } else {
                    self.borrowers.insert(
                        user,
                        Borrower {
                            address: user,
                            collateral: HashSet::new(),
                            debt: HashSet::from([log.reserve]),
                        },
                    );
                }
            });

        info!(
            "Got borrow logs from {} to {}",
            self.last_block_number, latest_block
        );

        self.get_supply_logs(self.last_block_number.into(), latest_block)
            .await?
            .into_iter()
            .for_each(|log| {
                let user = log.on_behalf_of;
                // fetch assets if user already a supplier
                if self.borrowers.contains_key(&user) {
                    let borrower = self.borrowers.get_mut(&user).unwrap();
                    borrower.collateral.insert(log.reserve);
                    return;
                } else {
                    self.borrowers.insert(
                        user,
                        Borrower {
                            address: user,
                            collateral: HashSet::from([log.reserve]),
                            debt: HashSet::new(),
                        },
                    );
                }
            });

        info!(
            "Got supply logs from {} to {}",
            self.last_block_number, latest_block
        );

        // write state cache to file
        info!("Write state cache to file {}", STATE_CACHE_FILE);
        let cache = StateCache {
            last_block_number: latest_block.as_u64(),
            borrowers: self.borrowers.clone(),
        };
        self.last_block_number = latest_block.as_u64();
        let mut file = File::create(STATE_CACHE_FILE)?;
        file.write_all(serde_json::to_string(&cache)?.as_bytes())?;

        Ok(())
    }

    // fetch all borrow events from the from_block to to_block
    async fn get_borrow_logs(&self, from_block: U64, to_block: U64) -> Result<Vec<BorrowFilter>> {
        let pool = Pool::<M>::new(self.config.pool_address, self.client.clone());

        let mut res = Vec::new();
        for start_block in
            (from_block.as_u64()..to_block.as_u64()).step_by(LOG_BLOCK_RANGE as usize)
        {
            let end_block = std::cmp::min(start_block + LOG_BLOCK_RANGE - 1, to_block.as_u64());
            pool.borrow_filter()
                .from_block(start_block)
                .to_block(end_block)
                .address(ValueOrArray::Value(self.config.pool_address))
                .query()
                .await?
                .into_iter()
                .for_each(|log| {
                    res.push(log);
                });
        }

        Ok(res)
    }

    // fetch all borrow events from the from_block to to_block
    async fn get_supply_logs(&self, from_block: U64, to_block: U64) -> Result<Vec<SupplyFilter>> {
        let pool = Pool::<M>::new(self.config.pool_address, self.client.clone());

        let mut res = Vec::new();
        for start_block in
            (from_block.as_u64()..to_block.as_u64()).step_by(LOG_BLOCK_RANGE as usize)
        {
            let end_block = std::cmp::min(start_block + LOG_BLOCK_RANGE - 1, to_block.as_u64());
            pool.supply_filter()
                .from_block(start_block)
                .to_block(end_block)
                .address(ValueOrArray::Value(self.config.pool_address))
                .query()
                .await?
                .into_iter()
                .for_each(|log| {
                    res.push(log);
                });
        }

        Ok(res)
    }

    async fn approve_tokens(&mut self) -> Result<()> {
        let liquidator = Liquidator::new(self.liquidator, self.client.clone());

        let mut nonce = self
            .client
            .get_transaction_count(
                self.client
                    .default_sender()
                    .ok_or(anyhow!("No connected sender"))?,
                None,
            )
            .await?;
        for token_address in self.tokens.keys() {
            let token = IERC20::new(token_address.clone(), self.client.clone());
            let allowance = token
                .allowance(self.liquidator, self.config.pool_address)
                .call()
                .await?;
            if allowance == U256::zero() {
                // TODO remove unwrap once we figure out whats broken
                liquidator
                    .approve_pool(*token_address)
                    .nonce(nonce)
                    .send()
                    .await
                    .map_err(|e| {
                        error!("approve failed: {:?}", e);
                        e
                    })?;
                nonce = nonce + 1;
            }
        }

        Ok(())
    }

    async fn update_token_configs(&mut self) -> Result<()> {
        let pool_data =
            IPoolDataProvider::<M>::new(self.config.pool_data_provider, self.client.clone());
        let all_tokens = pool_data.get_all_reserves_tokens().await?;
        let all_a_tokens = pool_data.get_all_a_tokens().await?;
        info!("all_tokens: {:?}", all_tokens);
        for (token, a_token) in zip(all_tokens, all_a_tokens) {
            let (decimals, ltv, threshold, bonus, reserve, _, _, _, _, _) = pool_data
                .get_reserve_configuration_data(token.token_address)
                .await?;
            let protocol_fee = pool_data
                .get_liquidation_protocol_fee(token.token_address)
                .await?;
            self.tokens.insert(
                token.token_address,
                TokenConfig {
                    address: token.token_address,
                    a_address: a_token.token_address,
                    decimals: decimals.low_u64(),
                    ltv: ltv.low_u64(),
                    liquidation_threshold: threshold.low_u64(),
                    liquidation_bonus: bonus.low_u64(),
                    reserve_factor: reserve.low_u64(),
                    protocol_fee: protocol_fee.low_u64(),
                },
            );
        }

        Ok(())
    }

    // 8 decimals of precision
    async fn get_asset_price_eth(&self, asset: &Address, pool_state: &PoolState) -> Result<U256> {
        // 1:1 for weth
        let weth_address = WETH_ADDRESS.parse::<Address>().unwrap();
        if asset.eq(&weth_address) {
            return Ok(U256::from(PRICE_ONE));
        }

        // usd / token
        let usd_price = pool_state
            .prices
            .get(asset)
            .ok_or(anyhow!("No price found for asset {}", asset.to_string()))?;
        // usd / eth
        let usd_price_eth = pool_state.prices.get(&weth_address).ok_or(anyhow!(
            "No price found for asset {}",
            weth_address.to_string()
        ))?;
        // usd / token * eth / usd = eth / token
        Ok(usd_price * U256::from(PRICE_ONE) / usd_price_eth)
    }

    async fn get_best_liquidation_op(&mut self) -> Result<Option<LiquidationOpportunity>> {
        let underwater = self.get_underwater_borrowers().await?;

        if underwater.len() == 0 {
            return Err(anyhow!("No underwater borrowers found"));
        }

        info!("Found {} underwater borrowers", underwater.len());
        let pool_data =
            IPoolDataProvider::<M>::new(self.config.pool_data_provider, self.client.clone());

        let mut best_bonus: U256 = U256::from(0);
        let mut best_op: Option<LiquidationOpportunity> = None;
        let pool_state = self.get_pool_state().await?;

        for (borrower, health_factor) in underwater {
            let borrower_details = self
                .borrowers
                .get(&borrower)
                .ok_or(anyhow!("Borrower not found"))?;

            for collateral_address in &borrower_details.collateral {
                for debt_address in &borrower_details.debt {
                    if collateral_address.ne(debt_address) {
                        if let Some(op) = self
                            .get_liquidation_opportunity(
                                &borrower,
                                collateral_address,
                                debt_address,
                                &pool_data,
                                &health_factor,
                                &pool_state,
                            )
                            .await
                            .map_err(|e| info!("Liquidation op failed {}", e))
                            .ok()
                        {
                            if op.profit_eth > best_bonus {
                                best_bonus = op.profit_eth;
                                best_op = Some(op);
                            }
                        }
                    }
                }
            }
        }

        Ok(best_op)
    }

    // Assumes there are WETH pairs for both collateral and debt asset types and that all pools have 500 fee
    // TODO: handle arbitrary pool fees and path
    fn get_swap_path(&self, collateral: &Address, debt: &Address) -> Result<Bytes> {
        let weth_address = WETH_ADDRESS.parse::<Address>()?;

        let pool_fee: u32 = 500;
        let pool_fee_encoded = pool_fee.to_be_bytes()[1..].to_vec(); // convert to uint24 by taking last 3 bytes only
        let mut path: Vec<Token> = Vec::new();

        // We first want to swap for debt
        path.push(Token::Address(*debt));
        path.push(Token::FixedBytes(pool_fee_encoded.clone()));

        // If neither the collateral or debt is WETH then we want to introduce an intermediate swap through WETH
        if collateral.ne(&weth_address) && debt.ne(&weth_address) {
            path.push(Token::Address(weth_address));
            path.push(Token::FixedBytes(pool_fee_encoded.clone()));
        }

        // Finally we want to use obtained collateral to pay the flash swap back
        path.push(Token::Address(*collateral));

        debug!("get_swap_path {:?}", path);

        let encoded_swap_path = encode_packed(&path)?;

        Ok(Bytes::from(encoded_swap_path))
    }

    // Returns aggregate swap call data as string
    async fn get_paraswap_call_data(
        &self,
        collateral: &Address,
        debt: &Address,
        amount: U256,
        is_buy: bool,
    ) -> Result<(Bytes, Address)> {
        let collateral_config = self
            .tokens
            .get(collateral)
            .ok_or(anyhow!("Failed to get collateral address"))?;
        let debt_config = self
            .tokens
            .get(debt)
            .ok_or(anyhow!("Failed to get debt address"))?;

        let src_decimals: Option<i32> = collateral_config.decimals.try_into().ok();

        let dest_decimals: Option<i32> = debt_config.decimals.try_into().ok();

        let amount = amount.to_string();

        let chain_id: i32 = self.chain_id.try_into()?;

        let side = if is_buy {
            prices_api::SwapSide::Buy
        } else {
            prices_api::SwapSide::Sell
        };

        // debt is the dest token because we want to repay the flash loan with collateral token
        let prices_params = prices_api::PricesGetParams {
            src_token: format!("{:?}", collateral),
            src_decimals,
            dest_token: format!("{:?}", debt),
            dest_decimals,
            amount: amount.clone(),
            side,
            network: Some(chain_id),
            user_address: Some(format!("{:?}", self.liquidator)),
            ..Default::default()
        };

        debug!("Prices params: {:?}", prices_params);

        let prices = prices_api::prices_get(&self.paraswap_api_config, prices_params).await?;

        let dest_amount = if is_buy {
            Some(amount.clone())
        } else {
            None
        };

        let src_amount = if is_buy {
            None
        } else {
            Some(amount.clone())
        };

        let transaction_params = transactions_api::TransactionsNetworkPostParams {
            transactions_request_payload: TransactionsRequestPayload {
                src_token: format!("{:?}", collateral),
                src_decimals,
                src_amount,
                dest_token: format!("{:?}", debt),
                dest_decimals,
                dest_amount,
                slippage: Some(9999), // Not concerned with slippage, liquidator will revert if insuffient dest token received from swap
                user_address: format!("{:?}", self.liquidator),
                receiver: Some(format!("{:?}", self.liquidator)),
                price_route: prices
                    .price_route
                    .ok_or(anyhow!("Missing paraswap price route response"))?,
                ..Default::default()
            },
            ignore_checks: Some(true),
            network: chain_id,
            ..Default::default()
        };

        debug!("Transaction params: {:?}", transaction_params);

        let transaction = transactions_api::transactions_network_post(
            &self.paraswap_api_config,
            transaction_params,
        )
        .await?;

        let call_data = Bytes::from_str(
            &transaction
                .data
                .ok_or(anyhow!("Missing paraswap transaction 'data' response"))?,
        )?;

        let augustus = Address::from_str(
            &transaction
                .to
                .ok_or(anyhow!("Missing paraswap transaction 'to' response"))?,
        )?;

        Ok((call_data, augustus))
    }

    async fn get_pool_state(&self) -> Result<PoolState> {
        let mut multicall = Multicall::<M>::new(
            self.client.clone(),
            Some(H160::from_str(MULTICALL3_ADDRESS)?),
        )
        .await?;
        let mut prices = HashMap::new();

        let price_oracle = IAaveOracle::<M>::new(self.config.oracle_address, self.client.clone());

        let mut flashloan_premium_total = U256::from(0);
        if self.dex_aggregator == DexAggregator::Paraswap {
            let pool = Pool::<M>::new(self.config.pool_address, self.client.clone());
            flashloan_premium_total = U256::from(pool.flashloan_premium_total().await?);
        }

        for token_address in self.tokens.keys() {
            multicall.add_call(price_oracle.get_asset_price(*token_address), false);
        }

        let result: Vec<U256> = multicall.call_array().await?;
        for (token_address, price) in zip(self.tokens.keys(), result) {
            prices.insert(*token_address, price);
        }
        multicall.clear_calls();

        Ok(PoolState {
            prices,
            flashloan_premium_total,
        })
    }

    async fn get_liquidation_opportunity(
        &self,
        borrower_address: &Address,
        collateral_address: &Address,
        debt_address: &Address,
        pool_data: &IPoolDataProvider<M>,
        health_factor: &U256,
        pool_state: &PoolState,
    ) -> Result<LiquidationOpportunity> {
        let collateral_asset_price = pool_state
            .prices
            .get(collateral_address)
            .ok_or(anyhow!("No collateral price"))?;
        let debt_asset_price = pool_state
            .prices
            .get(debt_address)
            .ok_or(anyhow!("No debt price"))?;
        let collateral_config = self
            .tokens
            .get(collateral_address)
            .ok_or(anyhow!("Failed to get collateral address"))?;
        let debt_config = self
            .tokens
            .get(debt_address)
            .ok_or(anyhow!("Failed to get debt address"))?;
        let collateral_unit = U256::from(10).pow(collateral_config.decimals.into());
        let debt_unit = U256::from(10).pow(debt_config.decimals.into());
        let liquidation_bonus = U256::from(collateral_config.liquidation_bonus);
        let a_token = IERC20::new(collateral_config.a_address.clone(), self.client.clone());

        let (_, stable_debt, variable_debt, _, _, _, _, _, _) = pool_data
            .get_user_reserve_data(*debt_address, *borrower_address)
            .await?;

        debug!("health_factor: {:?}", health_factor);
        let close_factor = if health_factor.gt(&U256::from(LIQUIDATION_CLOSE_FACTOR_THRESHOLD)) {
            U256::from(DEFAULT_LIQUIDATION_CLOSE_FACTOR)
        } else {
            U256::from(MAX_LIQUIDATION_CLOSE_FACTOR)
        };
        debug!("close_factor: {:?}", close_factor);

        let mut debt_to_cover =
            (stable_debt + variable_debt) * close_factor / U256::from(MAX_LIQUIDATION_CLOSE_FACTOR);
        let base_collateral = (debt_asset_price * debt_to_cover * collateral_unit)
            / (collateral_asset_price * debt_unit);
        let mut collateral_to_liquidate = percent_mul(base_collateral, liquidation_bonus);
        let user_collateral_balance = a_token.balance_of(*borrower_address).await?;

        debug!("debt_to_cover: {:?}", debt_to_cover);
        debug!(
            "collateral_to_liquidate: {:?}, user_collateral_balance: {:?}",
            collateral_to_liquidate, user_collateral_balance
        );

        if collateral_to_liquidate > user_collateral_balance {
            collateral_to_liquidate = user_collateral_balance;

            debug!("collateral_asset_price: {:?}, collateral_to_liquidate: {:?}, debt_unit: {:?}, debt_asset_price: {:?}, collateral_unit: {:?}, liquidation_bonus: {:?}", collateral_asset_price, collateral_to_liquidate, debt_unit, debt_asset_price, collateral_unit, liquidation_bonus);

            // round up debt to cover.
            let divisor = percent_div(debt_asset_price * collateral_unit, liquidation_bonus);
            debt_to_cover = ((collateral_asset_price * collateral_to_liquidate * debt_unit)
                + (divisor / 2))
                / divisor;
        }

        info!("debt_to_cover: {:?}", debt_to_cover);

        let mut op = LiquidationOpportunity {
            borrower: borrower_address.clone(),
            collateral: collateral_address.clone(),
            debt: debt_address.clone(),
            debt_to_cover,
            collateral_to_liquidate,
            profit_eth: U256::from(0),
        };

        if self.dex_aggregator == DexAggregator::Paraswap {
            let (collateral_gain, debt_gain) = self
                .build_liquidation_paraswap_call(&op, pool_state)
                .await?
                .call()
                .await?;

            let (weth_price_collateral, weth_price_debt) = join!(
                self.get_asset_price_eth(&op.collateral, pool_state),
                self.get_asset_price_eth(&op.debt, pool_state)
            );

            let profit = (collateral_gain * weth_price_collateral? / U256::from(PRICE_ONE))
                + (debt_gain * weth_price_debt? / U256::from(PRICE_ONE));

            op.profit_eth = profit;
        } else {
            let gain = self.build_liquidation_call(&op).await?.call().await?;

            let weth_price = self.get_asset_price_eth(&op.collateral, pool_state).await?;

            let profit =
                gain * I256::from_dec_str(&weth_price.to_string())? / I256::from(PRICE_ONE);

            op.profit_eth = match profit < I256::from(0) {
                true => U256::from(0),
                false => U256::try_from(profit)?,
            };
        }

        info!(
            "Found opportunity - borrower: {:?}, collateral: {:?}, debt: {:?}, profit_eth: {:?}",
            borrower_address, collateral_address, debt_address, op.profit_eth
        );

        Ok(op)
    }

    async fn build_liquidation_paraswap_call(
        &self,
        op: &LiquidationOpportunity,
        pool_state: &PoolState,
    ) -> Result<ContractCall<M, (U256, U256)>> {
        info!(
            "Build - borrower: {:?}, collateral: {:?}, debt: {:?}, debt_to_cover: {:?}, collateral_to_liquidate: {:?}, profit_eth: {:?}",
            op.borrower, op.collateral, op.debt, op.debt_to_cover, op.collateral_to_liquidate, op.profit_eth
        );

        let encoder = L2Encoder::new(self.config.l2_encoder, self.client.clone());
        let (data0, data1) = encoder
            .encode_liquidation_call(op.collateral, op.debt, op.borrower, op.debt_to_cover, false)
            .call()
            .await?;

        let liquidator_paraswap = LiquidatorParaswap::new(self.liquidator, self.client.clone());
        // let swap_dest_amount = percent_mul(
        //     op.debt_to_cover,
        //     pool_state.flashloan_premium_total + U256::from(PERCENT_HUNDRED),
        // );
        let (paraswap_call_data, paraswap_augustus) = self
            .get_paraswap_call_data(&op.collateral, &op.debt, op.collateral_to_liquidate, false)
            .await?;

        let contract_call = liquidator_paraswap.liquidate(
            LiquidationParams {
                collateral: op.collateral,
                debt: op.debt,
                debt_to_cover: op.debt_to_cover,
                liquidation_arg_1: data0,
                liquidation_arg_2: data1,
            },
            self.config.pool_address,
            SwapParams {
                augustus: paraswap_augustus,
                swap_call_data: paraswap_call_data,
            },
        );

        info!("Liquidation op contract call: {:?}", contract_call);

        return Ok(contract_call);
    }

    async fn build_liquidation_call(
        &self,
        op: &LiquidationOpportunity,
    ) -> Result<ContractCall<M, I256>> {
        info!(
            "Build - borrower: {:?}, collateral: {:?}, debt: {:?}, debt_to_cover: {:?}, collateral_to_liquidate: {:?}, profit_eth: {:?}",
            op.borrower, op.collateral, op.debt, op.debt_to_cover, op.collateral_to_liquidate, op.profit_eth
        );

        let encoder = L2Encoder::new(self.config.l2_encoder, self.client.clone());
        let (data0, data1) = encoder
            .encode_liquidation_call(op.collateral, op.debt, op.borrower, op.debt_to_cover, false)
            .call()
            .await?;

        let liquidator = Liquidator::new(self.liquidator, self.client.clone());

        let swap_path = self.get_swap_path(&op.collateral, &op.debt)?;

        let contract_call = liquidator.liquidate(
            op.collateral,
            op.debt_to_cover,
            data0,
            data1,
            Bytes::from(swap_path),
        );

        debug!("Liquidation op contract call: {:?}", contract_call);

        Ok(contract_call)
    }

    async fn build_liquidation(&self, op: &LiquidationOpportunity) -> Result<TypedTransaction> {
        let mut call = self.build_liquidation_call(op).await?;
        Ok(call.tx.set_chain_id(self.chain_id).clone())
    }
}

fn percent_mul(a: U256, bps: U256) -> U256 {
    (U256::from(5000) + (a * bps)) / U256::from(PERCENT_HUNDRED)
}

fn percent_div(a: U256, bps: U256) -> U256 {
    let half_bps = bps / U256::from(2);
    (half_bps + (a * U256::from(PERCENT_HUNDRED))) / bps
}
