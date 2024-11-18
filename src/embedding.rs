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
            .request(Method::POST, "/v1/embeddings")?
            .json(&request)
            .send()
            .await?;

        if response.status().is_success() {
            let embedding_response = response.json::<EmbeddingResponse>().await?;
            Ok(embedding_response)
        } else {
            Err(XaiError::Http(
                response.error_for_status().unwrap_err().to_string(),
            ))
        }
    }
}
