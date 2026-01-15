//! Error types for the Bybit SDK.

use std::fmt;

/// Errors that can occur when using the Bybit API client.
#[derive(Debug)]
pub enum Error {
    /// HTTP request failed
    Http(reqwest::Error),
    /// JSON parsing failed
    Json(serde_json::Error),
    /// Authentication failed
    Auth(String),
    /// API returned an error
    Api { code: i32, message: String },
    /// Invalid configuration
    Config(String),
    /// Session error
    Session(String),
    /// Cache error
    Cache(String),
}

impl fmt::Display for Error {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Error::Http(e) => write!(f, "HTTP error: {}", e),
            Error::Json(e) => write!(f, "JSON error: {}", e),
            Error::Auth(msg) => write!(f, "Authentication error: {}", msg),
            Error::Api { code, message } => write!(f, "API error {}: {}", code, message),
            Error::Config(msg) => write!(f, "Configuration error: {}", msg),
            Error::Session(msg) => write!(f, "Session error: {}", msg),
            Error::Cache(msg) => write!(f, "Cache error: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Http(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Json(err)
    }
}

/// Result type alias for convenience
pub type Result<T> = std::result::Result<T, Error>;
