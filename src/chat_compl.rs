//! Reference: https://docs.x.ai/api/endpoints#chat-completions

use crate::error::check_for_model_error;
use crate::error::XaiError;
use crate::traits::ChatCompletionsFetcher;
use crate::traits::ClientConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<HashMap<u32, f32>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<Usage>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
                stream: false,
                logprobs: None,
                top_p: None,
                top_logprobs: None,
                seed: None,
                user: None,
                logit_bias: None,
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

    pub fn frequency_penalty(mut self, frequency_penalty: f32) -> Self {
        self.request.frequency_penalty = Some(frequency_penalty);
        self
    }

    pub fn presence_penalty(mut self, presence_penalty: f32) -> Self {
        self.request.presence_penalty = Some(presence_penalty);
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

    pub fn stream(mut self, stream: bool) -> Self {
        self.request.stream = stream;
        self
    }

    pub fn logprobs(mut self, logprobs: bool) -> Self {
        self.request.logprobs = Some(logprobs);
        self
    }

    pub fn top_p(mut self, top_p: f32) -> Self {
        self.request.top_p = Some(top_p);
        self
    }

    pub fn top_logprobs(mut self, top_logprobs: u32) -> Self {
        self.request.top_logprobs = Some(top_logprobs);
        self
    }

    pub fn seed(mut self, seed: u32) -> Self {
        self.request.seed = Some(seed);
        self
    }

    pub fn user(mut self, user: String) -> Self {
        self.request.user = Some(user);
        self
    }

    pub fn logit_bias(mut self, logit_bias: HashMap<u32, f32>) -> Self {
        self.request.logit_bias = Some(logit_bias);
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
            let error_body = response.text().await.unwrap_or_else(|_| "".to_string());

            if let Some(model_error) = check_for_model_error(&error_body) {
                return Err(model_error);
            }

            Err(XaiError::Http(error_body))
        }
    }
}
