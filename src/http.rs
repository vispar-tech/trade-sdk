//! HTTP client module for Bybit API communication.

use async_trait::async_trait;
use hmac::{Hmac, Mac};
use reqwest::Client;
use sha2::Sha256;
use std::collections::HashMap;
use std::sync::Arc;

use crate::error::{Error, Result};
use crate::session::BybitSessionManager;
use crate::traits::HttpClient;
use crate::types::GenericResponse;

/// Domain constants.
const DOMAIN_MAIN: &str = "bybit";
const TLD_MAIN: &str = "com";

/// Masks sensitive headers for logging; truncates API key/sign values for safety.
fn mask_headers(headers: &reqwest::header::HeaderMap) -> std::collections::HashMap<String, String> {
    let mut masked = std::collections::HashMap::new();
    for (k, v) in headers.iter() {
        let key = k.as_str().to_lowercase();
        let value = v.to_str().unwrap_or("****");
        if key == "x-bapi-api-key" || key == "x-bapi-sign" {
            masked.insert(
                key.to_string(),
                format!("{}...", &value[..6.min(value.len())]),
            );
        } else {
            masked.insert(key.to_string(), value.to_string());
        }
    }
    masked
}

/// HTTP client for Bybit API (main, testnet, demo; NO bytick).
pub struct BybitHttpClient {
    client: Arc<Client>,
    base_url: String,
    api_key: Option<String>,
    api_secret: Option<String>,
    recv_window: u32,
    referral_id: Option<String>,
    uses_shared_session: bool,
}

impl BybitHttpClient {
    /// Creates a new Bybit HTTP client.
    ///
    /// # Parameters
    /// - `api_key`: Optional Bybit API key.
    /// - `api_secret`: Optional API secret.
    /// - `testnet`: Use Bybit testnet (`true`) or mainnet (`false`).
    /// - `demo`: Use demo trading API endpoints (`true`/`false`).
    /// - `recv_window`: Milliseconds for recvWindow (default 5000, allowed 10000-120000); controls how tolerant the API is to clock drift.
    /// - `referral_id`: Optional referral code.
    ///
    /// # Returns
    /// A constructed [`BybitHttpClient`] instance, using a shared reqwest client if [`SessionManager`] is initialized.
    ///
    /// # Example
    /// ```
    /// let client = BybitHttpClient::new(
    ///     Some("my_api_key".into()),
    ///     Some("my_secret".into()),
    ///     false, // testnet
    ///     false, // demo
    ///     5000,
    ///     None
    /// );
    /// ```
    pub fn new(
        api_key: Option<String>,
        api_secret: Option<String>,
        testnet: bool,
        demo: bool,
        recv_window: u32,
        referral_id: Option<String>,
    ) -> Self {
        // Determine subdomain based on testnet and demo flags
        let sub = match (demo, testnet) {
            (true, true) => "api-demo-testnet",
            (true, false) => "api-demo",
            (false, true) => "api-testnet",
            (false, false) => "api",
        };

        let base_url = format!("https://{}.{}.{}", sub, DOMAIN_MAIN, TLD_MAIN);

        // Check if shared session is initialized
        let (client, uses_shared_session) = if BybitSessionManager::is_initialized() {
            (BybitSessionManager::get_client(), true)
        } else {
            (
                Arc::new(
                    Client::builder()
                        .default_headers({
                            let mut headers = reqwest::header::HeaderMap::new();
                            headers.insert("Content-Type", "application/json".parse().unwrap());
                            headers.insert("Accept", "application/json".parse().unwrap());
                            headers
                        })
                        .build()
                        .unwrap(),
                ),
                false,
            )
        };

        Self {
            client,
            base_url,
            api_key,
            api_secret,
            recv_window,
            referral_id,
            uses_shared_session,
        }
    }

    /// Generates HMAC-SHA256 signature for Bybit V5 API.
    fn generate_signature(
        &self,
        api_key: &str,
        api_secret: &str,
        payload: &str,
        timestamp: i64,
    ) -> Result<String> {
        let param_str = format!("{}{}{}{}", timestamp, api_key, self.recv_window, payload);

        let mut mac = Hmac::<Sha256>::new_from_slice(api_secret.as_bytes())
            .map_err(|_| Error::Auth("Invalid API secret".to_string()))?;

        mac.update(param_str.as_bytes());
        let result = mac.finalize();
        let signature = result.into_bytes();

        Ok(hex::encode(signature))
    }

    /// Prepare HTTP payload string for signing (GET = query param string, others = sorted JSON).
    fn prepare_payload(
        method: &str,
        params: &HashMap<String, String>,
    ) -> String {
        if method == "GET" {
            if params.is_empty() {
                return String::new();
            }
            let mut sorted_params: Vec<_> = params.iter().collect();
            sorted_params.sort_by(|a, b| a.0.cmp(b.0));

            sorted_params
                .into_iter()
                .filter(|(_, v)| !v.is_empty())
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<_>>()
                .join("&")
        } else if params.is_empty() {
            "{}".to_string()
        } else {
            let mut sorted_params: Vec<_> = params.iter().collect();
            sorted_params.sort_by(|a, b| a.0.cmp(b.0));

            let filtered: HashMap<_, _> = sorted_params
                .into_iter()
                .filter(|(_, v)| !v.is_empty())
                .collect();

            serde_json::to_string(&filtered).unwrap_or_else(|_| "{}".to_string())
        }
    }

    /// Executes an HTTP request to the Bybit API and processes the response.
    ///
    /// **Private helper, not intended for direct use outside this module.**
    ///
    /// # Arguments
    /// - `method`: HTTP method as string, e.g., `"GET"`, `"POST"`, `"PUT"`, `"DELETE"`
    /// - `endpoint`: Relative API endpoint path (e.g. `"/v5/account/wallet-balance"`)
    /// - `params`: Request parameters as string map, or `None` for no params
    /// - `headers`: Optional extra headers to send
    /// - `auth`: Whether to sign request with API key
    ///
    /// # Returns
    /// - `Ok(GenericResponse)` if request is successful and API returns `ret_code == 0`
    /// - `Err(Error)` for HTTP, authentication or Bybit API error
    ///
    /// # Panics
    /// On reqwest build failures (should not happen in steady-state)
    async fn _async_request(
        &self,
        method: &str,
        endpoint: &str,
        params: Option<&HashMap<String, String>>,
        headers: Option<&HashMap<String, String>>,
        auth: bool,
    ) -> Result<GenericResponse> {
        // Build request args
        let params = params.unwrap_or(&HashMap::new()).clone();
        let timestamp = (std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64) as i64;

        let payload = Self::prepare_payload(method, &params);

        let url = if method == "GET" && !payload.is_empty() {
            format!("{}{}?{}", self.base_url, endpoint, payload)
        } else {
            format!("{}{}", self.base_url, endpoint)
        };

        let mut request = match method {
            "GET" => self.client.get(&url),
            "POST" => {
                let json_data: std::collections::HashMap<String, serde_json::Value> = params
                    .iter()
                    .map(|(k, v)| (k.clone(), serde_json::Value::String(v.clone())))
                    .collect();
                self.client.post(&url).json(&json_data)
            }
            "PUT" => {
                let json_data: std::collections::HashMap<String, serde_json::Value> = params
                    .iter()
                    .map(|(k, v)| (k.clone(), serde_json::Value::String(v.clone())))
                    .collect();
                self.client.put(&url).json(&json_data)
            }
            "DELETE" => self.client.delete(&url),
            _ => {
                return Err(Error::Config(format!(
                    "Unsupported HTTP method: {}",
                    method
                )))
            }
        };

        // Add custom headers if provided
        if let Some(headers) = headers {
            for (k, v) in headers {
                request = request.header(k, v);
            }
        }

        // Add referral header if present
        if let Some(ref referral_id) = self.referral_id {
            request = request.header("Referer", referral_id);
        }

        // Add authentication headers if required
        if auth {
            let api_key = self.api_key.as_ref().ok_or_else(|| {
                Error::Auth("API key required for authenticated requests".to_string())
            })?;
            let api_secret = self.api_secret.as_ref().ok_or_else(|| {
                Error::Auth("API secret required for authenticated requests".to_string())
            })?;

            let signature = self.generate_signature(api_key, api_secret, &payload, timestamp)?;

            request = request
                .header("X-BAPI-API-KEY", api_key)
                .header("X-BAPI-SIGN", signature)
                .header("X-BAPI-SIGN-TYPE", "2")
                .header("X-BAPI-TIMESTAMP", timestamp.to_string())
                .header("X-BAPI-RECV-WINDOW", self.recv_window.to_string());
        }

        // NOTE: Return our own error type, wrapping reqwest errors as Error::Http and API errors as Error::Api.
        let response = request.send().await.map_err(Error::Http)?;
        let status = response.status();

        if !status.is_success() {
            log::error!(
                "Request error: method={}, url={}, params={:?}, headers={:?}",
                method,
                url,
                params,
                mask_headers(response.headers())
            );
            // Even if status is error, still wrap as Error::Http
            return Err(Error::Http(response.error_for_status().unwrap_err()));
        }

        let api_response: GenericResponse = response.json().await.map_err(Error::Http)?;

        // Special handling for signature errors (retCode 10004)
        if api_response.ret_code == 10004 {
            if self.api_key.is_none() {
                // Skip logging if no API key
            } else {
                log::error!(
                    "Bybit signature error (retCode 10004): {}\nOrigin string is: '{}{}{}{}'. Params: {:?}",
                    api_response.ret_msg,
                    timestamp,
                    self.api_key.as_deref().unwrap_or(""),
                    self.recv_window,
                    payload,
                    &params
                );
            }
            return Err(Error::Api {
                code: api_response.ret_code,
                message: api_response.ret_msg,
            });
        }

        if api_response.ret_code != 0 {
            return Err(Error::Api {
                code: api_response.ret_code,
                message: api_response.ret_msg,
            });
        }

        Ok(api_response)
    }
}

#[async_trait]
impl HttpClient for BybitHttpClient {
    /// Sends an HTTP GET request to the Bybit API.
    ///
    /// # Arguments
    /// - `endpoint`: The endpoint path, e.g., `"/v5/account/wallet-balance"`.
    /// - `params`: Optional request parameters as a HashMap.
    /// - `auth`: If `true`, sends authentication headers (API key/secret required).
    ///
    /// # Returns
    /// A `GenericResponse` struct with the parsed API response.
    ///
    /// # Errors
    /// Returns an error if request fails or API returns a nonzero `ret_code`.
    async fn get(
        &self,
        endpoint: &str,
        params: Option<&HashMap<String, String>>,
        auth: bool,
    ) -> Result<GenericResponse> {
        self._async_request("GET", endpoint, params, None, auth)
            .await
    }

    /// Sends an HTTP POST request to the Bybit API.
    ///
    /// # Arguments
    /// - `endpoint`: The endpoint path.
    /// - `params`: Optional request parameters.
    /// - `auth`: Whether authentication is required.
    async fn post(
        &self,
        endpoint: &str,
        params: Option<&HashMap<String, String>>,
        auth: bool,
    ) -> Result<GenericResponse> {
        self._async_request("POST", endpoint, params, None, auth)
            .await
    }

    /// Sends an HTTP PUT request to the Bybit API.
    ///
    /// # Arguments
    /// - `endpoint`: The endpoint path.
    /// - `params`: Optional request parameters.
    /// - `auth`: Whether authentication is required.
    async fn put(
        &self,
        endpoint: &str,
        params: Option<&HashMap<String, String>>,
        auth: bool,
    ) -> Result<GenericResponse> {
        self._async_request("PUT", endpoint, params, None, auth)
            .await
    }

    /// Sends an HTTP DELETE request to the Bybit API.
    ///
    /// # Arguments
    /// - `endpoint`: The endpoint path.
    /// - `params`: Optional request parameters.
    /// - `auth`: Whether authentication is required.
    async fn delete(
        &self,
        endpoint: &str,
        params: Option<&HashMap<String, String>>,
        auth: bool,
    ) -> Result<GenericResponse> {
        self._async_request("DELETE", endpoint, params, None, auth)
            .await
    }
}

impl BybitHttpClient {
    /// Returns `true` if this client uses a shared HTTP session/reqwest client instance.
    ///
    /// If [`SessionManager`] is used to configure a shared client, this returns `true`.
    /// Otherwise, returns `false` (each instance owns its own reqwest client).
    pub fn uses_shared_session(&self) -> bool {
        self.uses_shared_session
    }
}
