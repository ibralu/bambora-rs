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
pub struct Detail {
    #[serde(rename = "field", skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl Detail {
    pub fn new() -> Detail {
        Detail {
            field: None,
            message: None,
        }
    }
}

