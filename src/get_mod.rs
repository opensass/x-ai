//! Reference: https://docs.x.ai/api/endpoints#get-model

use crate::error::check_for_model_error;
use crate::error::XaiError;
use crate::traits::{ClientConfig, ModelInfoFetcher};
use reqwest::Method;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfoResponse {
    pub created: u64,
    pub id: String,
    pub object: String,
    pub owned_by: String,
}

#[derive(Debug, Clone)]
pub struct ModelRequestBuilder<T: ClientConfig + Clone + Send + Sync> {
    client: T,
    model_id: String,
}

impl<T> ModelRequestBuilder<T>
where
    T: ClientConfig + Clone + Send + Sync,
{
    pub fn new(client: T, model_id: String) -> Self {
        Self { client, model_id }
    }
}

impl<T> ModelInfoFetcher for ModelRequestBuilder<T>
where
    T: ClientConfig + Clone + Send + Sync,
{
    async fn fetch_model_info(&self) -> Result<ModelInfoResponse, XaiError> {
        let url = format!("models/{}", self.model_id);

        let response = self.client.request(Method::GET, &url)?.send().await?;

        if response.status().is_success() {
            let body = response.text().await?;
            let chat_completion = serde_json::from_str::<ModelInfoResponse>(&body)?;
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
