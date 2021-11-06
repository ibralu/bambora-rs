# TokenResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**code** | Option<**String**> | Response code for the message. It can be ignored. | [optional]
**message** | Option<**String**> | Response message with any hints as to what might have gone wrong, if something went wrong. | [optional]
**token** | **String** | The token representing the credit card number that you will send to Beanstream for purchases. You will always receive a token, even if the request data is wrong or the card is invalid. | 
**version** | Option<**i32**> | Message version number. Can be ignored. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


