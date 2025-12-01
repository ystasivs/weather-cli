use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProviderError {
    #[error("request failed: {0}")]
    RequestFailed(String), // network / HTTP errors

    #[error("failed to parse response: {0}")]
    ParseError(String),    // JSON deserialization failed

    #[error("failed with msg from provider: {0}")]
    ProviderMsgError(String),

    #[error("provided error is out of range for provider: {0}")]
    DateIsOutOfRange(String),

    #[error("failed to convert to common format response from provider: {0}, e: {1}")]
    ConvertionError(String, String),    // other HTTP errors
}

pub type ProviderResult<T> = Result<T, ProviderError>;