//! Reference: https://docs.x.ai/api/endpoints#chat-completions

use crate::error::XaiError;
use crate::traits::ChatCompletionsFetcher;
use crate::traits::ClientConfig;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
    pub frequency_penalty: Option<f32>,
    pub presence_penalty: Option<f32>,
    pub n: Option<u32>,
    pub stop: Option<Vec<String>>,
    pub stream: Option<bool>,
    pub logprobs: Option<bool>,
    pub top_p: Option<f32>,
    pub top_logprobs: Option<u32>,
    pub seed: Option<u32>,
    pub user: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Option<Usage>,
    pub system_fingerprint: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Choice {
    pub index: u32,
    pub message: Message,
    pub finish_reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Debug, Clone)]
pub struct ChatCompletionsRequestBuilder<T: ClientConfig + Clone + Send + Sync> {
    client: T,
    request: ChatCompletionRequest,
}

impl<T> ChatCompletionsRequestBuilder<T>
where
    T: ClientConfig + Clone + Send + Sync,
{
    pub fn new(client: T, model: String, messages: Vec<Message>) -> Self {
        Self {
            client,
            request: ChatCompletionRequest {
                model,
                messages,
                temperature: None,
                max_tokens: None,
                frequency_penalty: None,
                presence_penalty: None,
                n: None,
                stop: None,
                stream: None,
                logprobs: None,
                top_p: None,
                top_logprobs: None,
                seed: None,
                user: None,
            },
        }
    }

    pub fn temperature(mut self, temperature: f32) -> Self {
        self.request.temperature = Some(temperature);
        self
    }

    pub fn max_tokens(mut self, max_tokens: u32) -> Self {
        self.request.max_tokens = Some(max_tokens);
        self
    }

    pub fn n(mut self, n: u32) -> Self {
        self.request.n = Some(n);
        self
    }

    pub fn stop(mut self, stop: Vec<String>) -> Self {
        self.request.stop = Some(stop);
        self
    }

    pub fn build(self) -> Result<ChatCompletionRequest, XaiError> {
        Ok(self.request)
    }
}

impl<T> ChatCompletionsFetcher for ChatCompletionsRequestBuilder<T>
where
    T: ClientConfig + Clone + Send + Sync,
{
    async fn create_chat_completion(
        &self,
        request: ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse, XaiError> {
        let response = self
            .client
            .request(reqwest::Method::POST, "chat/completions")?
            .json(&request)
            .send()
            .await?;

        if response.status().is_success() {
            let chat_completion = response.json::<ChatCompletionResponse>().await?;
            Ok(chat_completion)
        } else {
            Err(XaiError::Http(
                response.error_for_status().unwrap_err().to_string(),
            ))
        }
    }
}
