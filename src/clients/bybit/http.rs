//! HTTP client module for Bybit API communication.

use async_trait::async_trait;
use hmac::{Hmac, Mac};
use reqwest::Method;
use sha2::Sha256;
use std::collections::HashMap;

use crate::bybit::types::GenericResponse;
use crate::error::{Error, ExchangeResponseError, Result};
use crate::http::{BaseHttpClient, HttpClient, RequestArgs};

/// Domain constants.
const DOMAIN_MAIN: &str = "bybit";
const TLD_MAIN: &str = "com";

/// Masks sensitive headers for logging; truncates API key/sign values for safety.
fn mask_headers(headers: &HashMap<String, String>) -> HashMap<String, String> {
    let mut masked = HashMap::new();
    for (k, v) in headers.iter() {
        let key = k.to_lowercase();
        let value = v;
        if key == "x-bapi-api-key" || key == "x-bapi-sign" {
            let masked_val = format!("{}...", &value[..6.min(value.len())]);
            masked.insert(key, masked_val);
        } else {
            masked.insert(key, value.clone());
        }
    }
    masked
}

/// HTTP client for Bybit API (main, testnet, demo; NO bytick).
pub struct BybitHttpClient {
    base_client: BaseHttpClient,
    referral_id: Option<String>,
}

impl BybitHttpClient {
    /// Create a new Bybit HTTP client.
    ///
    /// * `api_key` - Optional API key.
    /// * `api_secret` - Optional API secret.
    /// * `testnet` - Use testnet if true, mainnet if false.
    /// * `demo` - Use demo trading endpoints if true.
    /// * `recv_window` - RecvWindow in ms.
    /// * `referral_id` - Optional referral code.
    pub fn new(
        api_key: Option<String>,
        api_secret: Option<String>,
        testnet: bool,
        demo: bool,
        recv_window: u32,
        referral_id: Option<String>,
    ) -> Result<Self> {
        let sub = match (demo, testnet) {
            (true, true) => "api-demo-testnet",
            (true, false) => "api-demo",
            (false, true) => "api-testnet",
            (false, false) => "api",
        };

        let base_url = format!("https://{}.{}.{}", sub, DOMAIN_MAIN, TLD_MAIN);

        let base_client = BaseHttpClient::new(base_url, api_key, api_secret, recv_window)?;

        Ok(Self {
            base_client,
            referral_id,
        })
    }

    /// Generates HMAC-SHA256 signature for Bybit V5 API.
    fn generate_signature(
        &self,
        api_key: &str,
        api_secret: &str,
        payload: &str,
        timestamp: i64,
    ) -> Result<String> {
        let param_str = format!(
            "{}{}{}{}",
            timestamp, api_key, self.base_client.recv_window, payload
        );

        let mut mac = Hmac::<Sha256>::new_from_slice(api_secret.as_bytes())
            .map_err(|_| Error::Auth("Invalid API secret".to_string()))?;

        mac.update(param_str.as_bytes());
        let result = mac.finalize();
        let signature = result.into_bytes();

        Ok(hex::encode(signature))
    }

    /// Prepare HTTP payload string for signing (GET = query param string, others = sorted JSON).
    fn prepare_payload(
        method: &Method,
        params: &HashMap<String, serde_json::Value>,
    ) -> String {
        if method == Method::GET {
            if params.is_empty() {
                return String::new();
            }
            let mut sorted_params: Vec<_> = params.iter().collect();
            sorted_params.sort_by(|a, b| a.0.cmp(b.0));

            sorted_params
                .into_iter()
                .filter_map(|(k, v)| {
                    // Skip empty-string values.
                    // We'll treat JSON null as absence.
                    match v {
                        serde_json::Value::Null => None,
                        // Strings: only add if not empty.
                        serde_json::Value::String(s) if s.is_empty() => None,
                        other => {
                            // For GET, according Bybit rules, everything should be flattened into a string
                            // Integer, bool, float types get .to_string()
                            // String just use as is
                            // Else, for objects/arrays, use compact JSON value
                            let sval = match other {
                                serde_json::Value::String(s) => s.clone(),
                                serde_json::Value::Number(n) => n.to_string(),
                                serde_json::Value::Bool(b) => b.to_string(),
                                _ => other.to_string(),
                            };
                            if sval.is_empty() {
                                None
                            } else {
                                Some(format!("{}={}", k, sval))
                            }
                        }
                    }
                })
                .collect::<Vec<_>>()
                .join("&")
        } else if params.is_empty() {
            "{}".to_string()
        } else {
            let mut sorted_params: Vec<_> = params.iter().collect();
            sorted_params.sort_by(|a, b| a.0.cmp(b.0));
            let filtered: serde_json::Map<String, serde_json::Value> = sorted_params
                .into_iter()
                .filter_map(|(k, v)| match v {
                    serde_json::Value::Null => None,
                    serde_json::Value::String(s) if s.is_empty() => None,
                    _ => Some((k.clone(), v.clone())),
                })
                .collect();
            serde_json::to_string(&filtered).unwrap_or_else(|_| "{}".to_string())
        }
    }

    pub fn is_shared_session_enabled(&self) -> bool {
        self.base_client.is_shared_session_enabled()
    }

    pub fn set_recv_window(
        &mut self,
        recv_window: u32,
    ) {
        self.base_client.set_recv_window(recv_window)
    }
}

#[async_trait]
impl HttpClient<GenericResponse> for BybitHttpClient {
    async fn build_request_args(
        &self,
        method: reqwest::Method,
        endpoint: &str,
        params: Option<&HashMap<String, serde_json::Value>>,
        auth: bool,
    ) -> Result<RequestArgs> {
        // Build request args
        let params = params.cloned().unwrap_or_default();
        let timestamp = (std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64) as i64;

        let payload = Self::prepare_payload(&method, &params);

        let url = if method == reqwest::Method::GET && !payload.is_empty() {
            format!("{}{}?{}", self.base_client.base_url, endpoint, payload)
        } else {
            format!("{}{}", self.base_client.base_url, endpoint)
        };

        // Prepare JSON data for POST/PUT requests
        let json = if method == reqwest::Method::POST || method == reqwest::Method::PUT {
            Some(serde_json::Value::Object(serde_json::Map::from_iter(
                params.iter().map(|(k, v)| (k.clone(), v.clone())),
            )))
        } else {
            None
        };

        // Prepare authentication headers if required
        let mut headers = HashMap::new();
        if auth {
            let api_key = self.base_client.api_key.as_ref().ok_or_else(|| {
                Error::Auth("API key required for authenticated requests".to_string())
            })?;
            let api_secret = self.base_client.api_secret.as_ref().ok_or_else(|| {
                Error::Auth("API secret required for authenticated requests".to_string())
            })?;

            let signature = self.generate_signature(api_key, api_secret, &payload, timestamp)?;

            headers.insert("X-BAPI-API-KEY".to_string(), api_key.clone());
            headers.insert("X-BAPI-SIGN".to_string(), signature);
            headers.insert("X-BAPI-SIGN-TYPE".to_string(), "2".to_string());
            headers.insert("X-BAPI-TIMESTAMP".to_string(), timestamp.to_string());
            headers.insert(
                "X-BAPI-RECV-WINDOW".to_string(),
                self.base_client.recv_window.to_string(),
            );
        }

        // Add referral header if present
        if let Some(ref referral_id) = self.referral_id {
            headers.insert("Referer".to_string(), referral_id.clone());
        }

        if log::log_enabled!(log::Level::Debug) {
            log::debug!(
                "Making async {:?} request to {} with params: {:?}",
                method,
                &url,
                &params,
            );
        }

        Ok(RequestArgs {
            url,
            headers,
            params: None,
            json,
            data: None,
        })
    }

    async fn async_request(
        &self,
        method: reqwest::Method,
        endpoint: &str,
        params: Option<&HashMap<String, serde_json::Value>>,
        auth: bool,
    ) -> Result<GenericResponse> {
        let request_args = self
            .build_request_args(method.clone(), endpoint, params, auth)
            .await?;

        let mut request = self
            .base_client
            .client
            .request(method.clone(), &request_args.url);

        if let Some(json) = &request_args.json {
            request = request.json(json);
        }

        for (k, v) in &request_args.headers {
            request = request.header(k, v);
        }

        println!("Request Args: {:?}", request_args);

        let response = request.send().await.map_err(Error::Http)?;
        let status = response.status();

        if !status.is_success() {
            log::error!(
									"HTTP error during async request: method={}, url={}, headers={:?}, status={}, response={:?}",
									method,
									&request_args.url,
									mask_headers(&request_args.headers),
									status,
									&response
							);
            return Err(Error::Http(response.error_for_status().unwrap_err()));
        }

        // First parse the response as serde_json::Value
        let value: serde_json::Value = response.json().await.map_err(Error::Http)?;
        let ret_code = value.get("retCode").and_then(|v| v.as_i64()).unwrap_or(0);

        if ret_code != 0 {
            let err = ExchangeResponseError::from(value);
            log::error!(
									"ExchangeResponseError during async request: method={}, url={}, headers={:?}, status={}, error={}",
									method,
									&request_args.url,
									mask_headers(&request_args.headers),
									status,
									err
							);
            return Err(Error::Exchange(err));
        }

        // It's ok, parse as GenericResponse
        let generic: GenericResponse = serde_json::from_value(value).map_err(Error::Json)?;
        Ok(generic)
    }
}
