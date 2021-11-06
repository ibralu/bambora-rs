pub use openapi::{apis, models};
use openapi::apis::configuration::Configuration;
use reqwest::Client;
use reqwest::header;

type BoxedError = dyn std::error::Error;


/// Returns new api config
pub fn new_config(passcode: String, header_map: Option<header::HeaderMap>) -> Result<Configuration, BoxedError> {
    let mut headers = if let Some(header_map) = header_map {
        header_map
    } else {
        header::HeaderMap::new();
    };
    let passcode_header = header::HeaderValue::from_str(&format!("Passcode {}", passcode)).map_err(|e| e.into())?;
    headers.insert("Authorization", passcode_header);
    let client = Client::builder().default_headers(headers).build().map_err(|e| e.into())?;
    let config = Configuration {
        client,
        ..Default::default()
    };

    Ok(config)
}
