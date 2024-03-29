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
pub struct Address {
    /// alphanumeric (64)
    #[serde(rename = "address_line1", skip_serializing_if = "Option::is_none")]
    pub address_line1: Option<String>,
    /// alphanumeric (64)
    #[serde(rename = "address_line2", skip_serializing_if = "Option::is_none")]
    pub address_line2: Option<String>,
    /// alphanumeric (32)
    #[serde(rename = "city", skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// characters (2)
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// alphanumeric (32)
    #[serde(rename = "email_address", skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    /// alphanumeric (64)
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// alphanumeric (32)
    #[serde(rename = "phone_number", skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /// alphanumeric (16)
    #[serde(rename = "postal_code", skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// characters (2)
    #[serde(rename = "province", skip_serializing_if = "Option::is_none")]
    pub province: Option<String>,
}

impl Address {
    pub fn new() -> Address {
        Address {
            address_line1: None,
            address_line2: None,
            city: None,
            country: None,
            email_address: None,
            name: None,
            phone_number: None,
            postal_code: None,
            province: None,
        }
    }
}


