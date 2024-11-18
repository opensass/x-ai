use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum XaiError {
    #[error("HTTP error: {0}")]
    Http(String),

    #[error("Missing API key. Please set an API key before making requests.")]
    MissingApiKey,

    #[error("Unexpected response format.")]
    UnexpectedResponseFormat,

    #[error("Other error: {0}")]
    Other(String),
}

impl From<reqwest::Error> for XaiError {
    fn from(err: reqwest::Error) -> Self {
        XaiError::Http(err.to_string())
    }
}
