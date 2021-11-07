# PaymentRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**amount** | **f64** | A decimal value in dollars, or relevant currency. digits(9) | 
**billing** | Option<[**crate::models::Address**](Address.md)> |  | [optional]
**card** | Option<[**crate::models::Card**](Card.md)> |  | [optional]
**comments** | Option<**String**> | alphanumeric (256) | [optional]
**custom** | Option<[**crate::models::Custom**](Custom.md)> |  | [optional]
**customer_ip** | Option<**String**> | alphanumeric (30) | [optional]
**language** | Option<**String**> | characters (3) | [optional]
**order_number** | Option<**String**> | A unique order number. alphanumeric(30) | [optional]
**payment_method** | **String** | One of (card, token, payment_profile, cash, cheque). characters(20) | 
**payment_profile** | Option<[**crate::models::ProfilePurchase**](ProfilePurchase.md)> |  | [optional]
**recurring_payment** | Option<**bool**> |  | [optional]
**shipping** | Option<[**crate::models::Address**](Address.md)> |  | [optional]
**term_url** | Option<**String**> | alphanumeric (256) | [optional]
**token** | Option<[**crate::models::TokenPurchase**](TokenPurchase.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


