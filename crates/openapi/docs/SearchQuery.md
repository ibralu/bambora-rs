# SearchQuery

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**criteria** | Option<[**Vec<crate::models::Criteria>**](Criteria.md)> | Optional search criteria. All criteria are ANDed together. | [optional]
**end_date** | **String** | The end date (inclusive) '2015-04-22T10:03:19' in the timezone of your merchant account. | 
**end_row** | **f32** | Used to page the results. 1-based. This should always be 1 larger than start_row. | 
**name** | **String** | Only accepts 2 values. Can be either 'Search' for all fields or 'TransHistoryMinimal' for a subset of the fields returned in the results. | 
**start_date** | **String** | The start date (inclusive) '2015-04-22T10:03:19' in the timezone of your merchant account. | 
**start_row** | **f32** | Used to page the results. 1-based | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


