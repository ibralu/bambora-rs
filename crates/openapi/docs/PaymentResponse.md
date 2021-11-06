# PaymentResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**approved** | Option<**i32**> | Approved = 1, declined = 0 | [optional]
**auth_code** | Option<**String**> | alphanumeric (32) | [optional]
**card** | Option<[**crate::models::CardPurchaseResponse**](CardPurchaseResponse.md)> |  | [optional]
**created** | Option<**String**> | alphanumeric (32) | [optional]
**id** | Option<**String**> | digits (9) | [optional]
**links** | Option<[**Vec<crate::models::Link>**](Link.md)> |  | [optional]
**message** | Option<**String**> | alphanumeric (256) | [optional]
**message_id** | Option<**String**> | digits (3) | [optional]
**order_number** | Option<**String**> | alphanumeric (32) | [optional]
**payment_method** | Option<**String**> | characters (16) | [optional]
**_type** | Option<**String**> | characters (16) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


