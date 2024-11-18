//! Reference: https://docs.x.ai/api/endpoints#list-language-models

use crate::error::check_for_model_error;
use crate::error::XaiError;
use crate::traits::{ClientConfig, ModelFetcher};

use reqwest::Method;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageModel {
    pub completion_text_token_price: u32,
    pub created: u64,
    pub id: String,
    pub input_modalities: Vec<String>,
    pub object: String,
    pub output_modalities: Vec<String>,
    pub owned_by: String,
    pub prompt_image_token_price: u32,
    pub prompt_text_token_price: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageModelListResponse {
    pub models: Vec<LanguageModel>,
}

#[derive(Debug, Clone)]
pub struct LanguageModelRequestBuilder<T: ClientConfig + Clone + Send + Sync> {
    client: T,
}

impl<T> LanguageModelRequestBuilder<T>
where
    T: ClientConfig + Clone + Send + Sync,
{
    pub fn new(client: T) -> Self {
        Self { client }
    }
}

impl<T> ModelFetcher for LanguageModelRequestBuilder<T>
where
    T: ClientConfig + Clone + Send + Sync,
{
    async fn fetch_model_info(&self) -> Result<LanguageModelListResponse, XaiError> {
        let response = self
            .client
            .request(Method::GET, "language-models")?
            .send()
            .await?;
        if response.status().is_success() {
            let chat_completion = response.json::<LanguageModelListResponse>().await?;
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
