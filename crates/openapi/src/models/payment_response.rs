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
pub struct PaymentResponse {
    /// Approved = 1, declined = 0
    #[serde(rename = "approved", skip_serializing_if = "Option::is_none")]
    pub approved: Option<i32>,
    /// alphanumeric (32)
    #[serde(rename = "auth_code", skip_serializing_if = "Option::is_none")]
    pub auth_code: Option<String>,
    #[serde(rename = "card", skip_serializing_if = "Option::is_none")]
    pub card: Option<Box<crate::models::CardPurchaseResponse>>,
    /// alphanumeric (32)
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// digits (9)
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<crate::models::Link>>,
    /// alphanumeric (256)
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// digits (3)
    #[serde(rename = "message_id", skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    /// alphanumeric (32)
    #[serde(rename = "order_number", skip_serializing_if = "Option::is_none")]
    pub order_number: Option<String>,
    /// characters (16)
    #[serde(rename = "payment_method", skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<String>,
    /// characters (16)
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
}

impl PaymentResponse {
    pub fn new() -> PaymentResponse {
        PaymentResponse {
            approved: None,
            auth_code: None,
            card: None,
            created: None,
            id: None,
            links: None,
            message: None,
            message_id: None,
            order_number: None,
            payment_method: None,
            _type: None,
        }
    }
}

