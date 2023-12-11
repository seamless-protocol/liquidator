# \PricesApi

All URIs are relative to *https://apiv5.paraswap.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**prices_get**](PricesApi.md#prices_get) | **GET** /prices | Request prices.



## prices_get

> crate::models::InlineResponse200 prices_get(src_token, dest_token, amount, side, src_decimals, dest_decimals, network, other_exchange_prices, include_dexs, exclude_dexs, include_contract_methods, exclude_contract_methods, user_address, route, partner)
Request prices.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**src_token** | **String** | Source Token Address or Token Symbol (for tokens from /tokens). | [required] |
**dest_token** | **String** | Destination Token Address or Token Symbol (for tokens from /tokens). | [required] |
**amount** | **String** | Amount in the Denomination of Source Token | [required] |
**side** | [**SwapSide**](.md) | Side of the swap. | [required] |
**src_decimals** | Option<**i32**> | Source Token Decimals; can be omitted if Symbol is provided for `srcToken`. |  |
**dest_decimals** | Option<**i32**> | Destination Token Decimals; can be omitted if Symbol is provided for `destToken`. |  |
**network** | Option<[**Network1**](.md)> | ID of the blockchain network. |  |
**other_exchange_prices** | Option<**bool**> | _If provided_, **others** object is filled in the response with price quotes from other exchanges (if available for comparison). |  |
**include_dexs** | Option<[**Vec<crate::models::Dexs>**](crate::models::Dexs.md)> | Comma Separated List of DEXs to include without spaces. |  |
**exclude_dexs** | Option<[**Vec<crate::models::Dexs>**](crate::models::Dexs.md)> | Comma Separated List of DEXs to exclude without spaces. |  |
**include_contract_methods** | Option<[**Vec<crate::models::ContractMethod>**](crate::models::ContractMethod.md)> | Comma Separated List of Contract Methods to include without spaces. |  |
**exclude_contract_methods** | Option<[**Vec<crate::models::ContractMethod>**](crate::models::ContractMethod.md)> | Comma Separated List of Contract Methods to exclude without spaces. |  |
**user_address** | Option<**String**> | User Wallet Address. |  |
**route** | Option<**String**> | Dash (-) separated list of tokens (addresses or symbols from /tokens) to comprise the price route. Max 4 tokens |  |
**partner** | Option<**String**> | partner string |  |

### Return type

[**crate::models::InlineResponse200**](inline_response_200.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

