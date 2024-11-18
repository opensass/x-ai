use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum XaiError {
    #[error("HTTP error: {0}")]
    Http(String),

    #[error("Network error: Unable to reach the server. Please check your connection.")]
    NetworkError,

    #[error("Missing API key. Please set an API key before making requests.")]
    MissingApiKey,

    #[error("Unexpected response format: {0}")]
    UnexpectedResponseFormat(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Serialization/Deserialization error: {0}")]
    SerdeError(String),

    #[error("The model does not exist or is not accessible: {0}")]
    ModelNotFoundError(String),

    #[error("Other error: {0}")]
    Other(String),
}

impl From<reqwest::Error> for XaiError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_connect() {
            XaiError::NetworkError
        } else if err.is_status() {
            XaiError::Http(err.status().map_or_else(
                || "Unknown HTTP status error".to_string(),
                |status| format!("HTTP error with status code: {}", status),
            ))
        } else {
            XaiError::Http(err.to_string())
        }
    }
}

impl From<serde_json::Error> for XaiError {
    fn from(err: serde_json::Error) -> Self {
        XaiError::SerdeError(err.to_string())
    }
}

pub fn check_for_model_error(response: &str) -> Option<XaiError> {
    if response.contains("model") && response.contains("does not exist") {
        Some(XaiError::ModelNotFoundError(response.to_string()))
    } else {
        None
    }
}
