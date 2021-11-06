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
pub struct ProfilePurchase {
    /// Which credit card to use. Starts at 1 for the first card. You must configure how many cards can be stored by visiting the profile options in the back office. digits(1)
    #[serde(rename = "card_id")]
    pub card_id: i32,
    /// Set to FALSE for pre-auth. digit(1) or boolean
    #[serde(rename = "complete", skip_serializing_if = "Option::is_none")]
    pub complete: Option<bool>,
    /// The payment profile ID. alphanumeric(32)
    #[serde(rename = "customer_code")]
    pub customer_code: String,
}

impl ProfilePurchase {
    pub fn new(card_id: i32, customer_code: String) -> ProfilePurchase {
        ProfilePurchase {
            card_id,
            complete: None,
            customer_code,
        }
    }
}

