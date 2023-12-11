# PriceRoute

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**block_number** | **i32** |  | 
**network** | [**crate::models::Network**](Network.md) |  | 
**src_token** | **String** | Source Token Address | 
**src_decimals** | **i32** |  | 
**src_amount** | **String** |  | 
**dest_token** | **String** | Destination Token Address | 
**dest_decimals** | **i32** |  | 
**dest_amount** | **String** |  | 
**best_route** | [**crate::models::OptimalRoute**](OptimalRoute.md) |  | 
**others** | Option<[**crate::models::PriceRouteOthers**](PriceRoute_others.md)> |  | [optional]
**gas_cost_usd** | **String** |  | 
**gas_cost** | **String** |  | 
**side** | [**crate::models::SwapSide**](SwapSide.md) |  | 
**token_transfer_proxy** | **String** |  | 
**contract_address** | **String** |  | 
**contract_method** | **String** |  | 
**src_usd** | **String** |  | 
**dest_usd** | **String** |  | 
**partner** | **String** |  | 
**partner_fee** | **i32** |  | 
**max_impact_reached** | **bool** |  | 
**hmac** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


