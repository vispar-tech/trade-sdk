//! Error types for the Trade SDK.

use std::fmt;

/// Error returned by an exchange, includes the full response object and a message.
/// This error is meant to make it easy to inspect/pretty-print an exchange API's error response.
#[derive(Debug, Clone)]
pub struct ExchangeResponseError {
    /// The complete exchange response, deserialized as a map.
    pub resp: serde_json::Value,
    /// Human-readable error message, best-effort extracted from the response.
    pub message: String,
}

impl ExchangeResponseError {
    /// Constructs a new `ExchangeResponseError` with the full response.
    pub fn new(resp: serde_json::Value) -> Self {
        let extracted_msg = Self::extract_message(&resp);
        Self {
            resp,
            message: extracted_msg,
        }
    }

    /// Try to extract a typical error message field from the response map.
    pub fn extract_message(resp: &serde_json::Value) -> String {
        for key in ["msg", "message", "error", "retMsg", "error_message"] {
            if let Some(val) = resp.get(key) {
                if let Some(s) = val.as_str() {
                    return s.to_string();
                }
            }
        }
        "No error message found in response.".to_string()
    }

    /// Nicely pretty-print the response.
    pub fn pretty_response(&self) -> String {
        match serde_json::to_string_pretty(&self.resp) {
            Ok(s) => s,
            Err(_) => format!("{:?}", self.resp),
        }
    }
}

impl From<serde_json::Value> for ExchangeResponseError {
    fn from(resp: serde_json::Value) -> Self {
        ExchangeResponseError::new(resp)
    }
}

impl fmt::Display for ExchangeResponseError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        writeln!(f, "ExchangeResponseError: {}", self.message)?;
        writeln!(f, "Response: {}", self.pretty_response())?;
        Ok(())
    }
}

impl std::error::Error for ExchangeResponseError {}

/// Top-level error type for all SDK and API errors.
///
/// Use pattern-matching to introspect the error kind or get access to the full exchange response.
#[derive(Debug)]
pub enum Error {
    /// An HTTP error occurred.
    Http(reqwest::Error),
    /// Failed to parse JSON.
    Json(serde_json::Error),
    /// Authentication failed.
    Auth(String),
    /// Error returned by the exchange, including the full response object.
    Exchange(ExchangeResponseError),
    /// Configuration error.
    Config(String),
    /// Validation error (input did not satisfy invariants or failed checks).
    Validation(String),
    /// Session error.
    Session(String),
    /// Cache error.
    Cache(String),
    /// Functionality not implemented.
    NotImplemented(String),
}

impl fmt::Display for Error {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Error::Http(e) => write!(f, "HTTP error: {e}"),
            Error::Json(e) => write!(f, "JSON error: {e}"),
            Error::Auth(msg) => write!(f, "Authentication error: {msg}"),
            Error::Exchange(ex) => write!(f, "{ex}"),
            Error::Config(msg) => write!(f, "Configuration error: {msg}"),
            Error::Validation(msg) => write!(f, "Validation error: {msg}"),
            Error::Session(msg) => write!(f, "Session error: {msg}"),
            Error::Cache(msg) => write!(f, "Cache error: {msg}"),
            Error::NotImplemented(msg) => write!(f, "Not implemented: {msg}"),
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

impl From<ExchangeResponseError> for Error {
    fn from(err: ExchangeResponseError) -> Self {
        Error::Exchange(err)
    }
}

/// Result type alias for convenience
pub type Result<T> = std::result::Result<T, Error>;
