//! Reference: https://docs.x.ai/api/endpoints#create-embeddings

use crate::error::check_for_model_error;
use crate::error::XaiError;
use crate::traits::{ClientConfig, EmbeddingFetcher};
use reqwest::Method;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingRequest {
    pub input: Vec<String>,
    pub model: String,
    pub encoding_format: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingResponse {
    pub data: Vec<EmbeddingData>,
    pub model: String,
    pub object: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingData {
    pub embedding: EmbeddingValue,
    pub index: u32,
    pub object: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EmbeddingValue {
    Float(Vec<f32>),
}

#[derive(Debug, Clone)]
pub struct EmbeddingRequestBuilder<T: ClientConfig + Clone + Send + Sync> {
    client: T,
    request: EmbeddingRequest,
}

impl<T> EmbeddingRequestBuilder<T>
where
    T: ClientConfig + Clone + Send + Sync,
{
    pub fn new(client: T, model: String, input: Vec<String>, encoding_format: String) -> Self {
        Self {
            client,
            request: EmbeddingRequest {
                input,
                model,
                encoding_format,
            },
        }
    }

    pub fn build(self) -> Result<EmbeddingRequest, XaiError> {
        Ok(self.request)
    }
}

impl<T> EmbeddingFetcher for EmbeddingRequestBuilder<T>
where
    T: ClientConfig + Clone + Send + Sync,
{
    async fn create_embedding(
        &self,
        request: EmbeddingRequest,
    ) -> Result<EmbeddingResponse, XaiError> {
        let response = self
            .client
            .request(Method::POST, "embeddings")?
            .json(&request)
            .send()
            .await?;

        if response.status().is_success() {
            let chat_completion = response.json::<EmbeddingResponse>().await?;
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
