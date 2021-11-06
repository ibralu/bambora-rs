/*
 * Bambora Payments
 *
 * https://api.na.bambora.com
 *
 * The version of the OpenAPI document: 1.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SearchQuery {
    /// Optional search criteria. All criteria are ANDed together.
    #[serde(rename = "criteria", skip_serializing_if = "Option::is_none")]
    pub criteria: Option<Vec<crate::models::Criteria>>,
    /// The end date (inclusive) '2015-04-22T10:03:19' in the timezone of your merchant account.
    #[serde(rename = "end_date")]
    pub end_date: String,
    /// Used to page the results. 1-based. This should always be 1 larger than start_row.
    #[serde(rename = "end_row")]
    pub end_row: f32,
    /// Only accepts 2 values. Can be either 'Search' for all fields or 'TransHistoryMinimal' for a subset of the fields returned in the results.
    #[serde(rename = "name")]
    pub name: String,
    /// The start date (inclusive) '2015-04-22T10:03:19' in the timezone of your merchant account.
    #[serde(rename = "start_date")]
    pub start_date: String,
    /// Used to page the results. 1-based
    #[serde(rename = "start_row")]
    pub start_row: f32,
}

impl SearchQuery {
    pub fn new(end_date: String, end_row: f32, name: String, start_date: String, start_row: f32) -> SearchQuery {
        SearchQuery {
            criteria: None,
            end_date,
            end_row,
            name,
            start_date,
            start_row,
        }
    }
}


