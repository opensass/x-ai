//! Reference: https://docs.x.ai/api/endpoints#api-key

use crate::error::check_for_model_error;
use crate::traits::ApiKeyFetcher;
use crate::{error::XaiError, traits::ClientConfig};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ApiKeyInfo {
    pub acls: Vec<String>,
    pub api_key_blocked: bool,
    pub api_key_disabled: bool,
    pub api_key_id: String,
    pub create_time: String,
    pub modified_by: String,
    pub modify_time: String,
    pub name: String,
    pub redacted_api_key: String,
    pub team_blocked: bool,
    pub team_id: String,
    pub user_id: String,
}

#[derive(Debug, Clone)]
pub struct ApiKeyRequestBuilder<T: ClientConfig + Clone + Send + Sync> {
    client: T,
}

impl<T> ApiKeyRequestBuilder<T>
where
    T: ClientConfig + Clone + Send + Sync,
{
    pub fn new(client: T) -> Self {
        Self { client }
    }
}

impl<T> ApiKeyFetcher for ApiKeyRequestBuilder<T>
where
    T: ClientConfig + Clone + Send + Sync,
{
    async fn fetch_api_key_info(&self) -> Result<ApiKeyInfo, XaiError> {
        let response = self
            .client
            .request(reqwest::Method::GET, "api-key")?
            .send()
            .await?;

        if response.status().is_success() {
            let chat_completion = response.json::<ApiKeyInfo>().await?;
            Ok(chat_completion)
        } else {
            let error_body = response.text().await.unwrap_or_else(|_| "".to_string());

            if let Some(model_error) = check_for_model_error(&error_body) {
                return Err(model_error);
            }

            Err(XaiError::Http(error_body))
        }
    }
}
