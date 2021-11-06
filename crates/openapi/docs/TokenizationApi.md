# \TokenizationApi

All URIs are relative to *https://api.na.bambora.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**scripts_tokenization_tokens_post**](TokenizationApi.md#scripts_tokenization_tokens_post) | **POST** /scripts/tokenization/tokens | Tokenize credit card



## scripts_tokenization_tokens_post

> crate::models::TokenResponse scripts_tokenization_tokens_post(token_request)
Tokenize credit card

NOTE that the full URL for this API call is https://www.beanstream.com/scripts/tokenization/tokens. Turn a credit card into a token using this service. This helps lessen your PCI scope by not passing the credit card information to your server. Instead you turn the card number into a token in the client app, then send the token to the server. When you send the token to Beanstream to make a payment, Beanstream then looks up the card number from that token and makes the payment. Tokens can also be used to create payment profiles.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token_request** | Option<[**TokenRequest**](TokenRequest.md)> |  |  |

### Return type

[**crate::models::TokenResponse**](TokenResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

