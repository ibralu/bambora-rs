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
pub struct Criteria {
    /// Refer to the table on this page for the searchable fields http://developer.beanstream.com/documentation/analyze-payments/search-specific-criteria/
    #[serde(rename = "field", skip_serializing_if = "Option::is_none")]
    pub field: Option<i32>,
    /// URL encoded comparators such as less than, greater than, equals... Refer to the table on this page for the full list of comparators http://developer.beanstream.com/documentation/analyze-payments/search-specific-criteria/
    #[serde(rename = "operator", skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    /// The value you want to match against.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl Criteria {
    pub fn new() -> Criteria {
        Criteria {
            field: None,
            operator: None,
            value: None,
        }
    }
}


