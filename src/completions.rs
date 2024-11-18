//! Reference: https://docs.x.ai/api/endpoints#completions

use crate::error::check_for_model_error;
use crate::error::XaiError;
use crate::traits::{ClientConfig, CompletionsFetcher};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionsRequest {
    pub model: String,
    pub prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub best_of: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub echo: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<HashMap<String, i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionsResponse {
    pub choices: Vec<Choice>,
    pub created: u64,
    pub id: String,
    pub model: String,
    pub object: String,
    pub system_fingerprint: Option<String>,
    pub usage: Option<Usage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Choice {
    pub index: u32,
    pub text: String,
    pub finish_reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Debug, Clone)]
pub struct CompletionsRequestBuilder<T: ClientConfig + Clone + Send + Sync> {
    client: T,
    request: CompletionsRequest,
}

impl<T> CompletionsRequestBuilder<T>
where
    T: ClientConfig + Clone + Send + Sync,
{
    pub fn new(client: T, model: String, prompt: String) -> Self {
        Self {
            client,
            request: CompletionsRequest {
                model,
                prompt,
                best_of: None,
                echo: None,
                frequency_penalty: None,
                logit_bias: None,
                logprobs: None,
                max_tokens: None,
                n: None,
                presence_penalty: None,
                seed: None,
                stop: None,
                stream: None,
                suffix: None,
                temperature: None,
                top_p: None,
                user: None,
            },
        }
    }

    pub fn best_of(mut self, best_of: u32) -> Self {
        self.request.best_of = Some(best_of);
        self
    }

    pub fn echo(mut self, echo: bool) -> Self {
        self.request.echo = Some(echo);
        self
    }

    pub fn frequency_penalty(mut self, frequency_penalty: f32) -> Self {
        self.request.frequency_penalty = Some(frequency_penalty);
        self
    }

    pub fn logit_bias(mut self, logit_bias: HashMap<String, i32>) -> Self {
        self.request.logit_bias = Some(logit_bias);
        self
    }

    pub fn logprobs(mut self, logprobs: u32) -> Self {
        self.request.logprobs = Some(logprobs);
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

    pub fn presence_penalty(mut self, presence_penalty: f32) -> Self {
        self.request.presence_penalty = Some(presence_penalty);
        self
    }

    pub fn seed(mut self, seed: u32) -> Self {
        self.request.seed = Some(seed);
        self
    }

    pub fn stop(mut self, stop: Vec<String>) -> Self {
        self.request.stop = Some(stop);
        self
    }

    pub fn stream(mut self, stream: bool) -> Self {
        self.request.stream = Some(stream);
        self
    }

    pub fn suffix(mut self, suffix: String) -> Self {
        self.request.suffix = Some(suffix);
        self
    }

    pub fn temperature(mut self, temperature: f32) -> Self {
        self.request.temperature = Some(temperature);
        self
    }

    pub fn top_p(mut self, top_p: f32) -> Self {
        self.request.top_p = Some(top_p);
        self
    }

    pub fn user(mut self, user: String) -> Self {
        self.request.user = Some(user);
        self
    }

    pub fn build(self) -> Result<CompletionsRequest, XaiError> {
        if self.request.model.trim().is_empty() {
            return Err(XaiError::Validation("Model is required".to_string()));
        }
        if self.request.prompt.trim().is_empty() {
            return Err(XaiError::Validation("Prompt is required".to_string()));
        }
        Ok(self.request)
    }
}

impl<T> CompletionsFetcher for CompletionsRequestBuilder<T>
where
    T: ClientConfig + Clone + Send + Sync,
{
    async fn create_completions(
        &self,
        request: CompletionsRequest,
    ) -> Result<CompletionsResponse, XaiError> {
        let response = self
            .client
            .request(Method::POST, "completions")?
            .json(&request)
            .send()
            .await?;

        if response.status().is_success() {
            let chat_completion = response.json::<CompletionsResponse>().await?;
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
