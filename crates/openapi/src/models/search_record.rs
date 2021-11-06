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
pub struct SearchRecord {
    #[serde(rename = "b_address1", skip_serializing_if = "Option::is_none")]
    pub b_address1: Option<String>,
    #[serde(rename = "b_address2", skip_serializing_if = "Option::is_none")]
    pub b_address2: Option<String>,
    #[serde(rename = "b_city", skip_serializing_if = "Option::is_none")]
    pub b_city: Option<String>,
    #[serde(rename = "b_country", skip_serializing_if = "Option::is_none")]
    pub b_country: Option<String>,
    #[serde(rename = "b_email", skip_serializing_if = "Option::is_none")]
    pub b_email: Option<String>,
    #[serde(rename = "b_name", skip_serializing_if = "Option::is_none")]
    pub b_name: Option<String>,
    #[serde(rename = "b_phone", skip_serializing_if = "Option::is_none")]
    pub b_phone: Option<String>,
    #[serde(rename = "b_postal", skip_serializing_if = "Option::is_none")]
    pub b_postal: Option<String>,
    #[serde(rename = "b_province", skip_serializing_if = "Option::is_none")]
    pub b_province: Option<String>,
    /// The PaymentProfile ID used in this transaction, if appropriate.
    #[serde(rename = "customer_code", skip_serializing_if = "Option::is_none")]
    pub customer_code: Option<String>,
    #[serde(rename = "message_id", skip_serializing_if = "Option::is_none")]
    pub message_id: Option<f32>,
    #[serde(rename = "message_text", skip_serializing_if = "Option::is_none")]
    pub message_text: Option<String>,
    #[serde(rename = "product_id", skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(rename = "product_name", skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
    #[serde(rename = "ref1", skip_serializing_if = "Option::is_none")]
    pub ref1: Option<String>,
    #[serde(rename = "ref2", skip_serializing_if = "Option::is_none")]
    pub ref2: Option<String>,
    #[serde(rename = "ref3", skip_serializing_if = "Option::is_none")]
    pub ref3: Option<String>,
    #[serde(rename = "ref4", skip_serializing_if = "Option::is_none")]
    pub ref4: Option<String>,
    #[serde(rename = "ref5", skip_serializing_if = "Option::is_none")]
    pub ref5: Option<String>,
    #[serde(rename = "row_id", skip_serializing_if = "Option::is_none")]
    pub row_id: Option<f32>,
    #[serde(rename = "s_address1", skip_serializing_if = "Option::is_none")]
    pub s_address1: Option<String>,
    #[serde(rename = "s_address2", skip_serializing_if = "Option::is_none")]
    pub s_address2: Option<String>,
    #[serde(rename = "s_city", skip_serializing_if = "Option::is_none")]
    pub s_city: Option<String>,
    #[serde(rename = "s_country", skip_serializing_if = "Option::is_none")]
    pub s_country: Option<String>,
    #[serde(rename = "s_email", skip_serializing_if = "Option::is_none")]
    pub s_email: Option<String>,
    #[serde(rename = "s_name", skip_serializing_if = "Option::is_none")]
    pub s_name: Option<String>,
    #[serde(rename = "s_phone", skip_serializing_if = "Option::is_none")]
    pub s_phone: Option<String>,
    #[serde(rename = "s_postal", skip_serializing_if = "Option::is_none")]
    pub s_postal: Option<String>,
    #[serde(rename = "s_province", skip_serializing_if = "Option::is_none")]
    pub s_province: Option<String>,
    #[serde(rename = "trn_amount", skip_serializing_if = "Option::is_none")]
    pub trn_amount: Option<f32>,
    #[serde(rename = "trn_approval_code", skip_serializing_if = "Option::is_none")]
    pub trn_approval_code: Option<String>,
    /// Address Verification Service
    #[serde(rename = "trn_avs_result", skip_serializing_if = "Option::is_none")]
    pub trn_avs_result: Option<String>,
    #[serde(rename = "trn_batch_no", skip_serializing_if = "Option::is_none")]
    pub trn_batch_no: Option<f32>,
    #[serde(rename = "trn_card_expiry", skip_serializing_if = "Option::is_none")]
    pub trn_card_expiry: Option<String>,
    #[serde(rename = "trn_card_owner", skip_serializing_if = "Option::is_none")]
    pub trn_card_owner: Option<String>,
    /// MC VI etc
    #[serde(rename = "trn_card_type", skip_serializing_if = "Option::is_none")]
    pub trn_card_type: Option<String>,
    #[serde(rename = "trn_completions", skip_serializing_if = "Option::is_none")]
    pub trn_completions: Option<f32>,
    #[serde(rename = "trn_cvd_result", skip_serializing_if = "Option::is_none")]
    pub trn_cvd_result: Option<f32>,
    /// 2015-04-22T17:03:19.323+0000
    #[serde(rename = "trn_date_time", skip_serializing_if = "Option::is_none")]
    pub trn_date_time: Option<String>,
    #[serde(rename = "trn_id", skip_serializing_if = "Option::is_none")]
    pub trn_id: Option<f32>,
    #[serde(rename = "trn_ip", skip_serializing_if = "Option::is_none")]
    pub trn_ip: Option<String>,
    /// The credit card with the middle digits redacted with X's
    #[serde(rename = "trn_masked_card", skip_serializing_if = "Option::is_none")]
    pub trn_masked_card: Option<String>,
    #[serde(rename = "trn_order_number", skip_serializing_if = "Option::is_none")]
    pub trn_order_number: Option<String>,
    #[serde(rename = "trn_payment_method", skip_serializing_if = "Option::is_none")]
    pub trn_payment_method: Option<String>,
    #[serde(rename = "trn_reference", skip_serializing_if = "Option::is_none")]
    pub trn_reference: Option<f32>,
    #[serde(rename = "trn_response", skip_serializing_if = "Option::is_none")]
    pub trn_response: Option<f32>,
    #[serde(rename = "trn_returns", skip_serializing_if = "Option::is_none")]
    pub trn_returns: Option<f32>,
    #[serde(rename = "trn_type", skip_serializing_if = "Option::is_none")]
    pub trn_type: Option<String>,
    #[serde(rename = "trn_voided", skip_serializing_if = "Option::is_none")]
    pub trn_voided: Option<f32>,
}

impl SearchRecord {
    pub fn new() -> SearchRecord {
        SearchRecord {
            b_address1: None,
            b_address2: None,
            b_city: None,
            b_country: None,
            b_email: None,
            b_name: None,
            b_phone: None,
            b_postal: None,
            b_province: None,
            customer_code: None,
            message_id: None,
            message_text: None,
            product_id: None,
            product_name: None,
            ref1: None,
            ref2: None,
            ref3: None,
            ref4: None,
            ref5: None,
            row_id: None,
            s_address1: None,
            s_address2: None,
            s_city: None,
            s_country: None,
            s_email: None,
            s_name: None,
            s_phone: None,
            s_postal: None,
            s_province: None,
            trn_amount: None,
            trn_approval_code: None,
            trn_avs_result: None,
            trn_batch_no: None,
            trn_card_expiry: None,
            trn_card_owner: None,
            trn_card_type: None,
            trn_completions: None,
            trn_cvd_result: None,
            trn_date_time: None,
            trn_id: None,
            trn_ip: None,
            trn_masked_card: None,
            trn_order_number: None,
            trn_payment_method: None,
            trn_reference: None,
            trn_response: None,
            trn_returns: None,
            trn_type: None,
            trn_voided: None,
        }
    }
}

