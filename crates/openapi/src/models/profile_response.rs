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
pub struct ProfileResponse {
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<f32>,
    /// the ID of the payment profile. You need to keep track of this value.
    #[serde(rename = "customer_code", skip_serializing_if = "Option::is_none")]
    pub customer_code: Option<String>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl ProfileResponse {
    pub fn new() -> ProfileResponse {
        ProfileResponse {
            code: None,
            customer_code: None,
            message: None,
        }
    }
}

