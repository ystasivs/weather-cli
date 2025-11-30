use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProviderError {
    #[error("request failed: {0}")]
    RequestFailed(String), // network / HTTP errors

    #[error("failed to parse response: {0}")]
    ParseError(String),    // JSON deserialization failed

    #[error("failed with msg from provider: {0}")]
    ProviderMsgError(String),

    #[error("unexpected status code {0}: {1}")]
    Other(u16, String),    // other HTTP errors
}

pub type ProviderResult<T> = std::result::Result<T, ProviderError>;