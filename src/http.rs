//! Generic async HTTP client primitives.
#![allow(dead_code)]

use async_trait::async_trait;
use reqwest::{Client, Method};

use std::collections::HashMap;
use std::sync::Arc;

use crate::error::{Error, Result};
use crate::session::SharedSessionManager;

/// HTTP request args (owned, ergonomic).
#[derive(Debug, Clone)]
pub struct RequestArgs {
    pub url: String,
    pub headers: HashMap<String, String>,
    pub params: Option<HashMap<String, String>>,
    pub json: Option<serde_json::Value>,
    pub data: Option<HashMap<String, String>>,
}

pub struct BaseHttpClient {
    pub base_url: String,
    pub api_key: Option<String>,
    pub api_secret: Option<String>,
    pub recv_window: u32,
    pub client: Arc<Client>,
    use_shared_session: bool,
}

impl BaseHttpClient {
    /// Create client; prefers session pool if available.
    pub fn new(
        base_url: String,
        api_key: Option<String>,
        api_secret: Option<String>,
        recv_window: u32,
    ) -> Result<Self> {
        if SharedSessionManager::is_initialized() {
            Ok(Self {
                base_url,
                api_key,
                api_secret,
                recv_window,
                client: SharedSessionManager::get_client(),
                use_shared_session: true,
            })
        } else {
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert(
                "Content-Type",
                "application/json"
                    .parse()
                    .map_err(|e| Error::Config(format!("Header parse: {e}")))?,
            );
            headers.insert(
                "Accept",
                "application/json"
                    .parse()
                    .map_err(|e| Error::Config(format!("Header parse: {e}")))?,
            );
            let client = Arc::new(
                Client::builder()
                    .default_headers(headers)
                    .pool_max_idle_per_host(50)
                    .build()
                    .map_err(Error::Http)?,
            );
            Ok(Self {
                base_url,
                api_key,
                api_secret,
                recv_window,
                client,
                use_shared_session: false,
            })
        }
    }

    pub fn set_recv_window(
        &mut self,
        recv_window: u32,
    ) {
        self.recv_window = recv_window;
    }

    pub fn is_shared_session_enabled(&self) -> bool {
        self.use_shared_session
    }
}

/// Async HTTP trait (owned argument style).
#[async_trait]
pub trait HttpClient<T>: Send + Sync
where
    T: serde::de::DeserializeOwned + Send,
{
    /// Build request arguments for composing an HTTP request.
    ///
    /// # Arguments
    /// * `method` - The HTTP method (GET, POST, PUT, etc).
    /// * `endpoint` - The API endpoint path.
    /// * `params` - Optional reference to query or body parameters as key/value pairs.
    /// * `auth` - Whether to include authentication info.
    ///
    /// # Returns
    /// Returns a [`Result`] containing [`RequestArgs`] for the request.
    async fn build_request_args(
        &self,
        method: Method,
        endpoint: &str,
        params: Option<&HashMap<String, serde_json::Value>>,
        auth: bool,
    ) -> Result<RequestArgs>;

    /// Perform an asynchronous HTTP request.
    ///
    /// # Arguments
    /// * `method` - The HTTP method.
    /// * `endpoint` - The API endpoint path.
    /// * `params` - Optional reference to parameters.
    /// * `auth` - Whether to include authentication info.
    ///
    /// # Returns
    /// Returns a [`Result`] containing the deserialized response of type `T`.
    async fn async_request(
        &self,
        method: Method,
        endpoint: &str,
        params: Option<&HashMap<String, serde_json::Value>>,
        auth: bool,
    ) -> Result<T>;

    /// Perform an HTTP GET request.
    ///
    /// # Arguments
    /// * `endpoint` - The API endpoint path.
    /// * `params` - Optional reference to query parameters.
    /// * `auth` - Whether to include authentication info.
    ///
    /// # Returns
    /// Returns a [`Result`] containing the deserialized response of type `T`.
    async fn get(
        &self,
        endpoint: &str,
        params: Option<&HashMap<String, serde_json::Value>>,
        auth: bool,
    ) -> Result<T> {
        self.async_request(Method::GET, endpoint, params, auth)
            .await
    }

    /// Perform an HTTP POST request.
    ///
    /// # Arguments
    /// * `endpoint` - The API endpoint path.
    /// * `params` - Optional reference to POST body parameters.
    /// * `auth` - Whether to include authentication info.
    ///
    /// # Returns
    /// Returns a [`Result`] containing the deserialized response of type `T`.
    async fn post(
        &self,
        endpoint: &str,
        params: Option<&HashMap<String, serde_json::Value>>,
        auth: bool,
    ) -> Result<T> {
        self.async_request(Method::POST, endpoint, params, auth)
            .await
    }

    /// Perform an HTTP PUT request.
    ///
    /// # Arguments
    /// * `endpoint` - The API endpoint path.
    /// * `params` - Optional reference to PUT body parameters.
    /// * `auth` - Whether to include authentication info.
    ///
    /// # Returns
    /// Returns a [`Result`] containing the deserialized response of type `T`.
    async fn put(
        &self,
        endpoint: &str,
        params: Option<&HashMap<String, serde_json::Value>>,
        auth: bool,
    ) -> Result<T> {
        self.async_request(Method::PUT, endpoint, params, auth)
            .await
    }

    /// Perform an HTTP DELETE request.
    ///
    /// # Arguments
    /// * `endpoint` - The API endpoint path.
    /// * `params` - Optional reference to query or body parameters.
    /// * `auth` - Whether to include authentication info.
    ///
    /// # Returns
    /// Returns a [`Result`] containing the deserialized response of type `T`.
    async fn delete(
        &self,
        endpoint: &str,
        params: Option<&HashMap<String, serde_json::Value>>,
        auth: bool,
    ) -> Result<T> {
        self.async_request(Method::DELETE, endpoint, params, auth)
            .await
    }
}
