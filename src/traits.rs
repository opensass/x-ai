#![allow(async_fn_in_trait)]

use crate::api_key::ApiKeyInfo;
use crate::error::XaiError;
use reqwest::{Method, RequestBuilder};

pub trait ClientConfig {
    fn set_api_key(&self, api_key: String);
    fn get_api_key(&self) -> Option<String>;
    fn request(&self, method: Method, endpoint: &str) -> Result<RequestBuilder, XaiError>;
}

pub trait ApiKeyFetcher {
    async fn fetch_api_key_info(&self) -> Result<ApiKeyInfo, XaiError>;
}
