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
pub struct TokenResponse {
    /// Response code for the message. It can be ignored.
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Response message with any hints as to what might have gone wrong, if something went wrong.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// The token representing the credit card number that you will send to Beanstream for purchases. You will always receive a token, even if the request data is wrong or the card is invalid.
    #[serde(rename = "token")]
    pub token: String,
    /// Message version number. Can be ignored.
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
}

impl TokenResponse {
    pub fn new(token: String) -> TokenResponse {
        TokenResponse {
            code: None,
            message: None,
            token,
            version: None,
        }
    }
}

