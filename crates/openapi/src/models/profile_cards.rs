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
pub struct ProfileCards {
    /// 1-indexed card ID. Total number of cards allowed is configured in your merchant account.
    #[serde(rename = "card_id", skip_serializing_if = "Option::is_none")]
    pub card_id: Option<String>,
    /// MC, VI etc. characters(2)
    #[serde(rename = "card_type", skip_serializing_if = "Option::is_none")]
    pub card_type: Option<String>,
    /// digits(2)
    #[serde(rename = "expiry_month", skip_serializing_if = "Option::is_none")]
    pub expiry_month: Option<String>,
    /// digits(2)
    #[serde(rename = "expiry_year", skip_serializing_if = "Option::is_none")]
    pub expiry_year: Option<String>,
    /// Will say DEF for the default card
    #[serde(rename = "function", skip_serializing_if = "Option::is_none")]
    pub function: Option<String>,
    /// digits(20)
    #[serde(rename = "number", skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
}

impl ProfileCards {
    pub fn new() -> ProfileCards {
        ProfileCards {
            card_id: None,
            card_type: None,
            expiry_month: None,
            expiry_year: None,
            function: None,
            number: None,
        }
    }
}


