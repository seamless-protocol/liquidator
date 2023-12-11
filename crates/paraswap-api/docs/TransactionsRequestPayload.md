# TransactionsRequestPayload

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**src_token** | **String** | Source Token Address. Only Token Symbol could be speciﬁed for tokens from `/tokens`. | 
**src_decimals** | Option<**i32**> | Source Token Decimals; can be omitted if Symbol is provided for `srcToken`. | [optional]
**dest_token** | **String** | Destination Token Address. Only Token Symbol could be speciﬁed for tokens from `/tokens`. | 
**dest_decimals** | Option<**i32**> | Destination Token Decimals; can be omitted if Symbol is provided for `destToken`. | [optional]
**src_amount** | Option<**i32**> | Amount in the Denomination of `srcToken` as returned from the `/prices` end-point. Required if `side=SELL`. Could only be ommitted if slippage & destAmount is provided when `side=BUY` | [optional]
**dest_amount** | Option<**i32**> | Amount in the Denomination of `destToken`  as returned from the `/prices` end-point.Required if `side=SELL`. Could only be ommitted if slippage & srcAmount is provided when `side=SELL` | [optional]
**slippage** | Option<**i32**> | Slippage percentage (represented in basis points). Eg: for 2.5% slippage, set the value to 0.025 * 10000 = 250; for 10% = 1000. <b>slippage</b> could be passed instead of `destAmount` when `side=SELL` or `srcAmount` when `side=BUY` | [optional]
**user_address** | **String** | Address of the Signer | 
**tx_origin** | Option<**String**> | Whenever msg.sender (userAddress) is different than the address calling the paraswap contract, `txOrigin` must be passed along with `userAddress`. | [optional]
**receiver** | Option<**String**> | Address of the Receiver. | [optional]
**partner_address** | Option<**String**> | Partner Address. If provided takes precedence over `partner` | [optional]
**partner_fee_percent** | Option<**i32**> | Used together with `partner` if provided. Should be parsed in Basis Points. Look at `slippage` parameter description to understand better. | [optional]
**partner** | Option<**String**> | Partner string. If `partnerAddress` not provided, partnerFeePercent is matched against known partners | [optional]
**permit** | Option<**String**> | Permit-hash (hex-string) to omit approving the user before swap. Helps in saving gas. | [optional]
**deadline** | Option<**i32**> | Timestamp (10 digit/seconds precision) till when the given transaction is valid. Eg: 1629214486. For a 5 minute, `deadline` could be calculated as `Date.now()/1000 + 300.` | [optional]
**price_route** | [**crate::models::PriceRoute**](PriceRoute.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


