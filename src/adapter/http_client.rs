use crate::adapter::generated_api::configuration::Configuration;
use crate::error::SdkError;
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct HttpClient {
    pub(crate) configuration: Configuration,
}

impl HttpClient {
    pub fn new(api_secret: impl Into<String>) -> Result<Self, SdkError> {
        let mut headers = reqwest::header::HeaderMap::new();
        let auth_val = format!("PortOne {}", api_secret.into());
        let mut header_val = reqwest::header::HeaderValue::from_str(&auth_val)?;
        header_val.set_sensitive(true);
        headers.insert(reqwest::header::AUTHORIZATION, header_val);

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .timeout(Duration::from_secs(30))
            .build()?;

        let mut configuration = Configuration::new();
        configuration.client = client;
        configuration.base_path = "https://api.portone.io".to_string();
        Ok(Self { configuration })
    }

    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.configuration.base_path = base_url.into();
        self
    }
}
