# \TransactionsApi

All URIs are relative to *https://apiv5.paraswap.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**transactions_network_post**](TransactionsApi.md#transactions_network_post) | **POST** /transactions/{network} | Build Transaction to be sent to the blockchain.



## transactions_network_post

> crate::models::InlineResponse2001 transactions_network_post(network, transactions_request_payload, gas_price, ignore_checks, ignore_gas_estimate, only_params)
Build Transaction to be sent to the blockchain.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**network** | [**Network**](.md) | ID of the network. (Mainnet - 1, Ropsten - 3, Polygon - 56, BSC - 137). | [required] |
**transactions_request_payload** | [**TransactionsRequestPayload**](TransactionsRequestPayload.md) | Checkout `schemas/TransactionsRequestPayload` to infer what parameters are required to be parsed in the responseBody. (<b>Note</b>: The priceRoute object should be directly parsed without any change.) | [required] |
**gas_price** | Option<**String**> | The set gas-price for the transaction in wei. |  |
**ignore_checks** | Option<**bool**> | Allows the API to skip performing onchain checks such as balances, allowances, as well as transaction simulations. <b>Note:</b> The response does not contain <b><u>gas</u></b> parameter when <i>ignoreChecks</i> is set to `true`.  |  |
**ignore_gas_estimate** | Option<**bool**> | Allows the API to skip gas checks <b>Note:</b> The response does not contain <b><u>gas</u></b> parameter when <i>ignoreGasEstimate</i> is set to `true`. |  |
**only_params** | Option<**bool**> | Allows the API to return the contract parameters only. |  |

### Return type

[**crate::models::InlineResponse2001**](inline_response_200_1.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

