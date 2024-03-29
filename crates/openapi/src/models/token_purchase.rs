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
pub struct TokenPurchase {
    /// the payment token you retrieved from the Legato service. alphanumeric(36)
    #[serde(rename = "code")]
    pub code: String,
    /// Set to FALSE for pre-auth
    #[serde(rename = "complete", skip_serializing_if = "Option::is_none")]
    pub complete: Option<bool>,
    /// Card holder name. alphanumeric(64)
    #[serde(rename = "name")]
    pub name: String,
}

impl TokenPurchase {
    pub fn new(code: String, name: String) -> TokenPurchase {
        TokenPurchase {
            code,
            complete: None,
            name,
        }
    }
}


