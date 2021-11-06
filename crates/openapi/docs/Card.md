# Card

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**complete** | Option<**bool**> | set to false for Pre-Authorize, and true to complete a payment | [optional][default to true]
**cvd** | Option<**String**> | Security code on the back of the credit card. This can be set to mandatory in the back office. digits(3 or 4) | [optional][default to 123]
**expiry_month** | **String** | eg. 02 for February. digits(2) | [default to 02]
**expiry_year** | **String** | eg. 15 for 2015. digits(2) | [default to 18]
**name** | **String** | Card holder name. alphanumeric(64) | 
**number** | **String** | Credit card number (PAN). digits(20) | [default to 5100000010001004]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


