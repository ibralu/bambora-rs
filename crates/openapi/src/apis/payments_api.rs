/*
 * Bambora Payments
 *
 * https://api.na.bambora.com
 *
 * The version of the OpenAPI document: 1.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`payments_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PaymentsPostError {
    Status400(crate::models::BamboraException),
    Status401(crate::models::BamboraException),
    Status402(crate::models::BamboraException),
    Status403(crate::models::BamboraException),
    Status405(crate::models::BamboraException),
    Status500(crate::models::BamboraException),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`payments_trans_id_completions_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PaymentsTransIdCompletionsPostError {
    Status400(crate::models::BamboraException),
    Status401(crate::models::BamboraException),
    Status402(crate::models::BamboraException),
    Status403(crate::models::BamboraException),
    Status405(crate::models::BamboraException),
    Status500(crate::models::BamboraException),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`payments_trans_id_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PaymentsTransIdGetError {
    Status400(crate::models::BamboraException),
    Status401(crate::models::BamboraException),
    Status402(crate::models::BamboraException),
    Status403(crate::models::BamboraException),
    Status405(crate::models::BamboraException),
    Status500(crate::models::BamboraException),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`payments_trans_id_returns_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PaymentsTransIdReturnsPostError {
    Status400(crate::models::BamboraException),
    Status401(crate::models::BamboraException),
    Status402(crate::models::BamboraException),
    Status403(crate::models::BamboraException),
    Status405(crate::models::BamboraException),
    Status500(crate::models::BamboraException),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`payments_trans_id_void_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PaymentsTransIdVoidPostError {
    Status400(crate::models::BamboraException),
    Status401(crate::models::BamboraException),
    Status402(crate::models::BamboraException),
    Status403(crate::models::BamboraException),
    Status405(crate::models::BamboraException),
    Status500(crate::models::BamboraException),
    UnknownValue(serde_json::Value),
}


/// Make a payment using credit card, cash, cheque, profile, or token. Each payment type has its own json definition passed in the body. For all payments you have the standard Billing, Shipping, Comments, etc. fields that are optional. Only the amount is required along with the payment data for card, cash, cheque, profile, and token. You must change the payment_method for each payment type. Credit Card - \"card\", Payment Profile - \"payment_profile\", Legato Token - \"token\", Cash - \"cash\", Cheque - \"cheque\", Interac - \"interac\"
pub async fn payments_post(configuration: &configuration::Configuration, payment_request: Option<crate::models::PaymentRequest>) -> Result<crate::models::PaymentResponse, Error<PaymentsPostError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/payments", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&payment_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PaymentsPostError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Complete a pre-authorized payment. The amount of the transaction to complete must be less than or equal to the original pre-auth amount. Complete must be set to true.
pub async fn payments_trans_id_completions_post(configuration: &configuration::Configuration, trans_id: f64, payment_request: Option<crate::models::PaymentRequest>) -> Result<crate::models::PaymentResponse, Error<PaymentsTransIdCompletionsPostError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/payments/{transId}/completions", local_var_configuration.base_path, transId=trans_id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&payment_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PaymentsTransIdCompletionsPostError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get a previous payment (transaction).
pub async fn payments_trans_id_get(configuration: &configuration::Configuration, trans_id: i32) -> Result<crate::models::Transaction, Error<PaymentsTransIdGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/payments/{transId}", local_var_configuration.base_path, transId=trans_id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PaymentsTransIdGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Return payment.
pub async fn payments_trans_id_returns_post(configuration: &configuration::Configuration, trans_id: f64, _return: crate::models::ModelReturn) -> Result<crate::models::PaymentResponse, Error<PaymentsTransIdReturnsPostError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/payments/{transId}/returns", local_var_configuration.base_path, transId=trans_id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&_return);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PaymentsTransIdReturnsPostError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Void a transaction. You can void payments, returns, pre-auths, and completions. It will cancel that transaction.
pub async fn payments_trans_id_void_post(configuration: &configuration::Configuration, trans_id: f64, void: crate::models::Void) -> Result<crate::models::PaymentResponse, Error<PaymentsTransIdVoidPostError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/payments/{transId}/void", local_var_configuration.base_path, transId=trans_id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&void);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PaymentsTransIdVoidPostError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}
