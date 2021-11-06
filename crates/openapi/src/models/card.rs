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
pub struct Card {
    /// set to false for Pre-Authorize, and true to complete a payment
    #[serde(rename = "complete", skip_serializing_if = "Option::is_none")]
    pub complete: Option<bool>,
    /// Security code on the back of the credit card. This can be set to mandatory in the back office. digits(3 or 4)
    #[serde(rename = "cvd", skip_serializing_if = "Option::is_none")]
    pub cvd: Option<String>,
    /// eg. 02 for February. digits(2)
    #[serde(rename = "expiry_month")]
    pub expiry_month: String,
    /// eg. 15 for 2015. digits(2)
    #[serde(rename = "expiry_year")]
    pub expiry_year: String,
    /// Card holder name. alphanumeric(64)
    #[serde(rename = "name")]
    pub name: String,
    /// Credit card number (PAN). digits(20)
    #[serde(rename = "number")]
    pub number: String,
}

impl Card {
    pub fn new(expiry_month: String, expiry_year: String, name: String, number: String) -> Card {
        Card {
            complete: None,
            cvd: None,
            expiry_month,
            expiry_year,
            name,
            number,
        }
    }
}


