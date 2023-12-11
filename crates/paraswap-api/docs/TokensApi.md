# \TokensApi

All URIs are relative to *https://apiv5.paraswap.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**tokens_get**](TokensApi.md#tokens_get) | **GET** /tokens | 
[**tokens_network_get**](TokensApi.md#tokens_network_get) | **GET** /tokens/{network} | 



## tokens_get

> crate::models::TokensList tokens_get()


alias for /tokens/1

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::TokensList**](TokensList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tokens_network_get

> crate::models::TokensList tokens_network_get(network)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**network** | [**Network**](.md) | ID of the network. (Mainnet - 1, Ropsten - 3, Polygon - 56, BSC - 137). | [required] |

### Return type

[**crate::models::TokensList**](TokensList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

