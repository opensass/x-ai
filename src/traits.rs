#![allow(async_fn_in_trait)]

use crate::api_key::ApiKeyInfo;
use crate::chat_compl::ChatCompletionRequest;
use crate::chat_compl::ChatCompletionResponse;
use crate::completions::CompletionsRequest;
use crate::completions::CompletionsResponse;
use crate::embedding::EmbeddingRequest;
use crate::embedding::EmbeddingResponse;
use crate::embedding_mod::EmbeddingModelsResponse;
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

pub trait ChatCompletionsFetcher {
    async fn create_chat_completion(
        &self,
        request: ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse, XaiError>;
}

pub trait CompletionsFetcher {
    async fn create_completions(
        &self,
        request: CompletionsRequest,
    ) -> Result<CompletionsResponse, XaiError>;
}

pub trait EmbeddingFetcher {
    async fn create_embedding(
        &self,
        request: EmbeddingRequest,
    ) -> Result<EmbeddingResponse, XaiError>;
}

pub trait EmbeddingModelsFetcher {
    async fn list_embedding_models(&self) -> Result<EmbeddingModelsResponse, XaiError>;
}
