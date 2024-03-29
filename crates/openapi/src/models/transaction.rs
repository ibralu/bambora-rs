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
pub struct Transaction {
    #[serde(rename = "adjusted_by", skip_serializing_if = "Option::is_none")]
    pub adjusted_by: Option<Vec<crate::models::Adjustment>>,
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[serde(rename = "approved", skip_serializing_if = "Option::is_none")]
    pub approved: Option<bool>,
    #[serde(rename = "auth_code", skip_serializing_if = "Option::is_none")]
    pub auth_code: Option<String>,
    #[serde(rename = "batch_number", skip_serializing_if = "Option::is_none")]
    pub batch_number: Option<String>,
    #[serde(rename = "billing", skip_serializing_if = "Option::is_none")]
    pub billing: Option<Box<crate::models::Address>>,
    #[serde(rename = "card", skip_serializing_if = "Option::is_none")]
    pub card: Option<Box<crate::models::CardGetTransactionResponse>>,
    #[serde(rename = "comments", skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// The date it was created '2015-04-22T10:03:19.323-07:00'
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(rename = "custom", skip_serializing_if = "Option::is_none")]
    pub custom: Option<Box<crate::models::Custom>>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<crate::models::Link>>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "message_id", skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i32>,
    #[serde(rename = "order_number", skip_serializing_if = "Option::is_none")]
    pub order_number: Option<String>,
    #[serde(rename = "payment_method", skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<String>,
    #[serde(rename = "shipping", skip_serializing_if = "Option::is_none")]
    pub shipping: Option<Box<crate::models::Address>>,
    #[serde(rename = "total_completions", skip_serializing_if = "Option::is_none")]
    pub total_completions: Option<f64>,
    #[serde(rename = "total_refunds", skip_serializing_if = "Option::is_none")]
    pub total_refunds: Option<f64>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
}

impl Transaction {
    pub fn new() -> Transaction {
        Transaction {
            adjusted_by: None,
            amount: None,
            approved: None,
            auth_code: None,
            batch_number: None,
            billing: None,
            card: None,
            comments: None,
            created: None,
            custom: None,
            id: None,
            links: None,
            message: None,
            message_id: None,
            order_number: None,
            payment_method: None,
            shipping: None,
            total_completions: None,
            total_refunds: None,
            _type: None,
        }
    }
}


