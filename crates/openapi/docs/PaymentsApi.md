# \PaymentsApi

All URIs are relative to *https://api.na.bambora.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**payments_post**](PaymentsApi.md#payments_post) | **POST** /payments | Make Payment
[**payments_trans_id_completions_post**](PaymentsApi.md#payments_trans_id_completions_post) | **POST** /payments/{transId}/completions | Complete pre-auth
[**payments_trans_id_get**](PaymentsApi.md#payments_trans_id_get) | **GET** /payments/{transId} | Get payment
[**payments_trans_id_returns_post**](PaymentsApi.md#payments_trans_id_returns_post) | **POST** /payments/{transId}/returns | Return payment
[**payments_trans_id_void_post**](PaymentsApi.md#payments_trans_id_void_post) | **POST** /payments/{transId}/void | Void Transaction



## payments_post

> crate::models::PaymentResponse payments_post(payment_request)
Make Payment

Make a payment using credit card, cash, cheque, profile, or token. Each payment type has its own json definition passed in the body. For all payments you have the standard Billing, Shipping, Comments, etc. fields that are optional. Only the amount is required along with the payment data for card, cash, cheque, profile, and token. You must change the payment_method for each payment type. Credit Card - \"card\", Payment Profile - \"payment_profile\", Legato Token - \"token\", Cash - \"cash\", Cheque - \"cheque\", Interac - \"interac\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payment_request** | Option<[**PaymentRequest**](PaymentRequest.md)> |  |  |

### Return type

[**crate::models::PaymentResponse**](PaymentResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payments_trans_id_completions_post

> crate::models::PaymentResponse payments_trans_id_completions_post(trans_id, payment_request)
Complete pre-auth

Complete a pre-authorized payment. The amount of the transaction to complete must be less than or equal to the original pre-auth amount. Complete must be set to true.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**trans_id** | **f64** | The transaction id. | [required] |
**payment_request** | Option<[**PaymentRequest**](PaymentRequest.md)> |  |  |

### Return type

[**crate::models::PaymentResponse**](PaymentResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payments_trans_id_get

> crate::models::Transaction payments_trans_id_get(trans_id)
Get payment

Get a previous payment (transaction).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**trans_id** | **i32** | The transaction id. | [required] |

### Return type

[**crate::models::Transaction**](Transaction.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payments_trans_id_returns_post

> crate::models::PaymentResponse payments_trans_id_returns_post(trans_id, _return)
Return payment

Return payment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**trans_id** | **f64** | The transaction id. | [required] |
**_return** | [**ModelReturn**](ModelReturn.md) |  | [required] |

### Return type

[**crate::models::PaymentResponse**](PaymentResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payments_trans_id_void_post

> crate::models::PaymentResponse payments_trans_id_void_post(trans_id, void)
Void Transaction

Void a transaction. You can void payments, returns, pre-auths, and completions. It will cancel that transaction.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**trans_id** | **f64** | The transaction id to void. | [required] |
**void** | [**Void**](Void.md) |  | [required] |

### Return type

[**crate::models::PaymentResponse**](PaymentResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

