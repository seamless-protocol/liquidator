///`LiquidationParams(address,address,uint256,bytes32,bytes32)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct LiquidationParams {
    pub collateral: ::ethers::core::types::Address,
    pub debt: ::ethers::core::types::Address,
    pub debt_to_cover: ::ethers::core::types::U256,
    pub liquidation_arg_1: [u8; 32],
    pub liquidation_arg_2: [u8; 32],
}
///`SwapParams(address,bytes)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct SwapParams {
    pub augustus: ::ethers::core::types::Address,
    pub swap_call_data: ::ethers::core::types::Bytes,
}
