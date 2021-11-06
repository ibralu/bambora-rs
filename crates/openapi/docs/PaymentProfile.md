# PaymentProfile

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_ref** | Option<**String**> |  | [optional]
**billing** | Option<[**crate::models::Address**](Address.md)> |  | [optional]
**card** | Option<[**crate::models::GetProfileDefaultCard**](GetProfileDefaultCard.md)> |  | [optional]
**code** | Option<**i32**> | digits(2) | [optional]
**custom** | Option<[**crate::models::Custom**](Custom.md)> |  | [optional]
**customer_code** | Option<**String**> | alphanumeric(32) | [optional]
**language** | Option<**String**> | characters(2) | [optional]
**last_transaction** | Option<**String**> | date and time alphanumeric(24) | [optional]
**message** | Option<**String**> | alphanumeric(64) | [optional]
**modified_date** | Option<**String**> | date and time it was last modified alphanumeric(24) | [optional]
**profile_group** | Option<**String**> |  | [optional]
**status** | Option<**String**> | characters(1) | [optional]
**velocity_group** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


