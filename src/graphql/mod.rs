// Pull in the schema we registered in build.rs
#[cynic::schema("aave_v3")]
mod schema {}

#[derive(cynic::QueryVariables, Debug)]
pub struct ActiveBorrowersVariables<'a> {
    pub first: i32,
    pub last: &'a str,
    pub zero: BigInt,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "ActiveBorrowersVariables")]
pub struct ActiveBorrowers {
    #[arguments(first: $first, where: { and: [{ id_gt: $last }, { reserves_: { currentATokenBalance_gt: $zero, usageAsCollateralEnabledOnUser: true } }, { reserves_: { currentTotalDebt_gt: $zero } }] })]
    pub users: Vec<User>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct User {
    pub id: cynic::Id,
    pub e_mode_category_id: Option<EmodeCategory>,
    pub reserves: Vec<UserReserve>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct UserReserve {
    pub id: cynic::Id,
    pub reserve: Reserve,
    pub current_total_debt: BigInt,
    #[cynic(rename = "currentATokenBalance")]
    pub current_atoken_balance: BigInt,
    #[cynic(rename = "scaledATokenBalance")]
    pub scaled_atoken_balance: BigInt,
    pub usage_as_collateral_enabled_on_user: bool,
    pub stable_borrow_rate: BigInt,
    pub scaled_variable_debt: BigInt,
    pub principal_stable_debt: BigInt,
    pub stable_borrow_last_update_timestamp: i32,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct Reserve {
    pub id: cynic::Id,
    pub decimals: i32,
    pub underlying_asset: Bytes,
    pub symbol: String,
    pub name: String,
    pub a_token: SubToken,
    #[cynic(rename = "baseLTVasCollateral")]
    pub base_ltvas_collateral: BigInt,
    pub price: PriceOracleAsset,
    pub reserve_liquidation_threshold: BigInt,
    pub e_mode: Option<EmodeCategory2>,
    pub usage_as_collateral_enabled: bool,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct SubToken {
    pub id: cynic::Id,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct PriceOracleAsset {
    pub price_in_eth: BigInt,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "EModeCategory")]
pub struct EmodeCategory2 {
    pub liquidation_threshold: BigInt,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "EModeCategory")]
pub struct EmodeCategory {
    pub id: cynic::Id,
}

#[derive(cynic::Scalar, Debug, Clone)]
pub struct BigInt(pub String);

#[derive(cynic::Scalar, Debug, Clone)]
pub struct Bytes(pub String);

