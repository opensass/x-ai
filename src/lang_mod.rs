//! Reference: https://docs.x.ai/api/endpoints#get-language-model

use crate::error::check_for_model_error;
use crate::error::XaiError;
use crate::traits::{ClientConfig, GetModelFetcher};
use reqwest::Method;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageModelDetailResponse {
    pub completion_text_token_price: u32,
    pub created: u64,
    pub id: String,
    pub input_modalities: Vec<String>,
    pub object: String,
    pub output_modalities: Vec<String>,
    pub owned_by: String,
    pub prompt_image_token_price: u32,
    pub prompt_text_token_price: u32,
    pub version: String,
}

#[derive(Debug, Clone)]
pub struct LanguageModelDetailRequestBuilder<T: ClientConfig + Clone + Send + Sync> {
    client: T,
    model_id: String,
}

impl<T> LanguageModelDetailRequestBuilder<T>
where
    T: ClientConfig + Clone + Send + Sync,
{
    pub fn new(client: T, model_id: String) -> Self {
        Self { client, model_id }
    }

    pub fn build(self) -> Result<String, XaiError> {
        if self.model_id.is_empty() {
            Err(XaiError::Validation("Model ID cannot be empty".to_string()))
        } else {
            Ok(self.model_id)
        }
    }
}

impl<T> GetModelFetcher for LanguageModelDetailRequestBuilder<T>
where
    T: ClientConfig + Clone + Send + Sync,
{
    async fn fetch_model_info(&self) -> Result<LanguageModelDetailResponse, XaiError> {
        let url = format!("language-models/{}", self.model_id);

        let response = self.client.request(Method::GET, &url)?.send().await?;

        if response.status().is_success() {
            let chat_completion = response.json::<LanguageModelDetailResponse>().await?;
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
