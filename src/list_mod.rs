//! Reference: https://docs.x.ai/api/endpoints#list-models

use crate::error::check_for_model_error;
use crate::error::XaiError;
use crate::traits::{ClientConfig, ListModelFetcher};
use reqwest::Method;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReducedModel {
    pub created: u64,
    pub id: String,
    pub object: String,
    pub owned_by: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReducedModelListResponse {
    pub data: Vec<ReducedModel>,
    pub object: String,
}

#[derive(Debug, Clone)]
pub struct ReducedModelListRequestBuilder<T: ClientConfig + Clone + Send + Sync> {
    client: T,
}

impl<T> ReducedModelListRequestBuilder<T>
where
    T: ClientConfig + Clone + Send + Sync,
{
    pub fn new(client: T) -> Self {
        Self { client }
    }
}

impl<T> ListModelFetcher for ReducedModelListRequestBuilder<T>
where
    T: ClientConfig + Clone + Send + Sync,
{
    async fn fetch_model_info(&self) -> Result<ReducedModelListResponse, XaiError> {
        let response = self.client.request(Method::GET, "models")?.send().await?;

        if response.status().is_success() {
            let body = response.text().await?;
            let chat_completion = serde_json::from_str::<ReducedModelListResponse>(&body)?;
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
