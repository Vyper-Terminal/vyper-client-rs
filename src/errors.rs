use thiserror::Error;

#[derive(Error, Debug)]
pub enum VyperError {
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("Deserialization error: {0}")]
    DeserializeError(#[from] serde_json::Error),

    #[error("Authentication error: {0}")]
    AuthenticationError(String),

    #[error("Rate limit error: {message}")]
    RateLimitError {
        message: String,
        retry_after: Option<f64>,
    },

    #[error("Server error: {0}")]
    ServerError(String),

    #[error("API error: {0} (Status: {1})")]
    ApiError(String, u16),

    #[error("Websocket error: {message}")]
    WebsocketError {
        message: String,
        status_code: Option<u16>,
        connection_info: Option<String>,
    },
}

impl VyperError {
    pub fn websocket_error<S: Into<String>>(message: S, status_code: Option<u16>, connection_info: Option<String>) -> Self {
        VyperError::WebsocketError {
            message: message.into(),
            status_code,
            connection_info,
        }
    }
}

impl From<&str> for VyperError {
    fn from(s: &str) -> Self {
        VyperError::WebsocketError {
            message: s.to_string(),
            status_code: None,
            connection_info: None,
        }
    }
}