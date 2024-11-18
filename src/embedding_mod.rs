//! Reference: https://docs.x.ai/api/endpoints#list-embedding-models

use crate::error::XaiError;
use crate::traits::{ClientConfig, EmbeddingModelsFetcher};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingModelsResponse {
    pub models: Vec<EmbeddingModel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingModel {
    pub created: u64,
    pub id: String,
    pub input_modalities: Vec<String>,
    pub object: String,
    pub owned_by: String,
    pub prompt_image_token_price: u64,
    pub prompt_text_token_price: u64,
    pub version: String,
}

impl<T> EmbeddingModelsFetcher for T
where
    T: ClientConfig + Send + Sync,
{
    async fn list_embedding_models(&self) -> Result<EmbeddingModelsResponse, XaiError> {
        let response = self
            .request(reqwest::Method::GET, "embedding-models")?
            .send()
            .await?;

        if response.status().is_success() {
            let models_response = response.json::<EmbeddingModelsResponse>().await?;
            Ok(models_response)
        } else {
            Err(XaiError::Http(
                response.error_for_status().unwrap_err().to_string(),
            ))
        }
    }
}
