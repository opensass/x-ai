use crate::error::XaiError;
use crate::traits::ClientConfig;
use crate::XAI_V1_URL;
use reqwest::{Client as HttpClient, Method, RequestBuilder};
use std::sync::{Arc, RwLock};

#[derive(Clone, Debug)]
pub struct XaiClient {
    http_client: Arc<HttpClient>,
    api_key: Arc<RwLock<Option<String>>>,
    base_url: String,
}

impl XaiClient {
    pub fn builder() -> XaiClientBuilder {
        XaiClientBuilder::default()
    }
}

impl ClientConfig for XaiClient {
    fn set_api_key(&self, api_key: String) {
        let mut key = self.api_key.write().unwrap();
        *key = Some(api_key);
    }

    fn get_api_key(&self) -> Option<String> {
        self.api_key.read().unwrap().clone()
    }

    fn request(&self, method: Method, endpoint: &str) -> Result<RequestBuilder, XaiError> {
        let api_key = self.get_api_key().ok_or(XaiError::MissingApiKey)?;

        let url = format!("{}/{}", self.base_url, endpoint);
        let builder = self
            .http_client
            .request(method, &url)
            .header("Authorization", format!("Bearer {}", api_key));
        Ok(builder)
    }
}

#[derive(Default, Debug)]
pub struct XaiClientBuilder {
    base_url: Option<String>,
}

impl XaiClientBuilder {
    pub fn base_url(mut self, url: &str) -> Self {
        self.base_url = Some(url.to_string());
        self
    }

    pub fn build(self) -> Result<XaiClient, XaiError> {
        Ok(XaiClient {
            http_client: Arc::new(HttpClient::new()),
            api_key: Arc::new(RwLock::new(None)),
            base_url: self.base_url.unwrap_or_else(|| XAI_V1_URL.to_string()),
        })
    }
}
