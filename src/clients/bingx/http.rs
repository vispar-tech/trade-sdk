//! HTTP client module for BingX API communication.

use async_trait::async_trait;
use hmac::{Hmac, Mac};
use reqwest::Method;
use sha2::Sha256;
use std::collections::HashMap;

use crate::bingx::types::GenericResponse;
use crate::error::{Error, ExchangeResponseError, Result};
use crate::http::{BaseHttpClient, HttpClient, RequestArgs};
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use std::collections::hash_map::Entry;

/// Masks sensitive headers for logging; truncates API key/sign values for safety.
fn mask_headers(headers: &HashMap<String, String>) -> HashMap<String, String> {
    let mut masked = HashMap::new();
    for (k, v) in headers.iter() {
        let key = k.to_lowercase();
        let value = v;
        if key == "x-bx-apikey" {
            let masked_val = format!("{}...", &value[..6.min(value.len())]);
            masked.insert(key, masked_val);
        } else {
            masked.insert(key, value.clone());
        }
    }
    masked
}

/// Mask the `signature` query parameter in a BingX API URL for safe logging.
/// Replaces the signature value with "***" in the URL.
fn mask_signature(url: &str) -> String {
    let sig = "signature=";
    if let Some(i) = url.find(sig) {
        let start = i + sig.len();
        if let Some(amp) = url[start..].find('&') {
            // signature is followed by another parameter
            format!("{}***{}", &url[..start], &url[start + amp..])
        } else {
            // signature is last or only param
            format!("{}***", &url[..start])
        }
    } else {
        url.to_owned()
    }
}

/// HTTP client for BingX API (main, testnet, demo).
pub struct BingxHttpClient {
    base_client: BaseHttpClient,
}

impl BingxHttpClient {
    /// Initialize a new BingxHttpClient.
    ///
    /// # Arguments
    /// * `api_key` - Trading API key (optional)
    /// * `api_secret` - Trading API secret (optional)
    /// * `demo` - Use vst (testnet) instead of mainnet
    /// * `recv_window` - Receive window in milliseconds (default 5000)
    pub fn new(
        api_key: Option<String>,
        api_secret: Option<String>,
        demo: bool,
        recv_window: u32,
    ) -> Result<Self> {
        let base_url = if demo {
            "https://open-api-vst.bingx.com".to_string()
        } else {
            "https://open-api.bingx.com".to_string()
        };
        let base_client = BaseHttpClient::new(base_url, api_key, api_secret, recv_window)?;
        Ok(Self { base_client })
    }

    /// Bingx V5 signature (API v5).
    fn generate_signature(
        &self,
        api_secret: &str,
        payload: &str,
    ) -> String {
        let mut mac =
            Hmac::<Sha256>::new_from_slice(api_secret.as_bytes()).expect("Invalid API secret");
        mac.update(payload.as_bytes());
        let signature = mac.finalize().into_bytes();
        hex::encode(signature)
    }

    /// Prepare BingX payload and URL-encoded payload, as per exchange rules.
    ///
    /// Returns a tuple: (payload_for_signature, url_encoded_payload_for_query)
    fn prepare_payload(
        &self,
        method: &Method,
        params: &mut HashMap<String, serde_json::Value>,
        timestamp: i64,
    ) -> (String, Option<String>) {
        // Helper to turn a serde_json::Value into a string for BingX params
        // (number, float, bool, string all must stringify as normal; skip Null, convert arrays/objects as json)
        fn value_to_str(v: &serde_json::Value) -> String {
            match v {
                serde_json::Value::String(s) => s.clone(),
                serde_json::Value::Number(n) => n.to_string(),
                serde_json::Value::Bool(b) => b.to_string(),
                serde_json::Value::Null => "".to_string(),
                serde_json::Value::Array(_) | serde_json::Value::Object(_) => v.to_string(),
            }
        }

        if method == Method::GET {
            // Sort params by key and check if any value contains struct/array indicators
            let param_count = params.len();
            let mut params_vec: Vec<(&str, String)> = Vec::with_capacity(param_count);
            let mut contains_struct = false;

            for (k, v) in params.iter() {
                let s = value_to_str(v);
                if !contains_struct && (s.contains('{') || s.contains('[')) {
                    contains_struct = true;
                }
                params_vec.push((k.as_str(), s));
            }

            params_vec.sort_by(|a, b| a.0.cmp(b.0));

            // Build signature string (plain, sorted, with timestamp)
            let mut params_str = String::with_capacity(64 + param_count * 24);
            for (i, (k, v)) in params_vec.iter().enumerate() {
                if i > 0 {
                    params_str.push('&');
                }
                params_str.push_str(k);
                params_str.push('=');
                params_str.push_str(v);
            }
            if !params_str.is_empty() {
                params_str.push_str("&timestamp=");
            } else {
                params_str.push_str("timestamp=");
            }
            params_str.push_str(&timestamp.to_string());

            // Build URL-encoded query string if needed
            let mut url_params_str = String::with_capacity(params_str.len());
            for (i, (k, v)) in params_vec.iter().enumerate() {
                if i > 0 {
                    url_params_str.push('&');
                }
                url_params_str.push_str(k);
                url_params_str.push('=');
                if contains_struct {
                    url_params_str.push_str(&utf8_percent_encode(v, NON_ALPHANUMERIC).to_string());
                } else {
                    url_params_str.push_str(v);
                }
            }
            if !url_params_str.is_empty() {
                url_params_str.push_str("&timestamp=");
            } else {
                url_params_str.push_str("timestamp=");
            }
            url_params_str.push_str(&timestamp.to_string());

            (params_str, Some(url_params_str))
        } else {
            // For POST/other: ensure "timestamp" is present (without overwriting)
            if let Entry::Vacant(e) = params.entry("timestamp".to_owned()) {
                e.insert(serde_json::Value::String(timestamp.to_string()));
            }

            // Sort params by key (as required by BingX)
            let param_count = params.len();
            let mut params_vec: Vec<(&str, String)> = Vec::with_capacity(param_count);
            for (k, v) in params.iter() {
                params_vec.push((k.as_str(), value_to_str(v)));
            }
            params_vec.sort_by(|a, b| a.0.cmp(b.0));

            // Build signature string (plain key=val pairs)
            let mut params_str = String::with_capacity(64 + param_count * 24);
            for (i, (k, v)) in params_vec.iter().enumerate() {
                if i > 0 {
                    params_str.push('&');
                }
                params_str.push_str(k);
                params_str.push('=');
                params_str.push_str(v);
            }
            (params_str, None)
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
impl HttpClient<GenericResponse> for BingxHttpClient {
    async fn build_request_args(
        &self,
        method: reqwest::Method,
        endpoint: &str,
        params: Option<&HashMap<String, serde_json::Value>>,
        auth: bool,
    ) -> Result<RequestArgs> {
        // Clone input parameters if any, else create empty HashMap
        let mut params = params.cloned().unwrap_or_else(HashMap::new);
        let mut headers = HashMap::new();

        // Get timestamp in ms since Unix epoch
        let timestamp = (std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis()) as i64;

        // Insert API key header if auth
        if auth {
            let api_key = self.base_client.api_key.as_ref().ok_or_else(|| {
                Error::Auth("API key must be set for authenticated requests.".to_string())
            })?;
            self.base_client.api_secret.as_ref().ok_or_else(|| {
                Error::Auth("API secret must be set for authenticated requests.".to_string())
            })?;
            headers.insert("X-BX-APIKEY".to_owned(), api_key.clone());
        }

        // Always insert recvWindow from base config
        params.insert(
            "recvWindow".to_owned(),
            serde_json::Value::Number(self.base_client.recv_window.into()),
        );

        // Prepare signature string and url-filtered params
        let (req_payload, req_url_params) = self.prepare_payload(&method, &mut params, timestamp);

        // Generate signature if auth required
        let signature = if auth {
            let api_secret = self.base_client.api_secret.as_ref().ok_or_else(|| {
                Error::Auth("API secret must be set for authenticated requests.".to_string())
            })?;
            Some(self.generate_signature(api_secret, &req_payload))
        } else {
            None
        };

        let base_req_url = format!("{}{}", self.base_client.base_url, endpoint);

        // Compose actual request url and JSON body
        let (req_url, req_json) = if method == reqwest::Method::GET {
            // GET: signature on URL params
            let mut req_url = match req_url_params {
                Some(ref url_params) => format!("{}?{}", base_req_url, url_params),
                None => base_req_url.clone(),
            };
            if let Some(signature) = &signature {
                if req_url.contains('?') {
                    req_url.push_str("&signature=");
                } else {
                    req_url.push_str("?signature=");
                }
                req_url.push_str(signature);
            }
            (req_url, None)
        } else {
            // POST or others: create a new params map for JSON body, insert signature & timestamp
            let mut json_params = params.clone();
            json_params.insert(
                "timestamp".to_owned(),
                serde_json::Value::Number(timestamp.into()),
            );
            if let Some(signature) = &signature {
                json_params.insert(
                    "signature".to_owned(),
                    serde_json::Value::String(signature.clone()),
                );
            }
            (
                base_req_url.clone(),
                Some(serde_json::Value::Object(json_params.into_iter().collect())),
            )
        };

        if log::log_enabled!(log::Level::Debug) {
            log::debug!(
                "Making async {:?} request to {} with params: {:?}",
                method,
                &base_req_url,
                params,
            );
        }

        Ok(RequestArgs {
            url: req_url,
            headers,
            params: None,
            json: req_json,
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

        let response = request.send().await.map_err(Error::Http)?;
        let status = response.status();

        if !status.is_success() {
            log::error!(
                "HTTP error during async request: method={}, url={}, headers={:?}, status={}, response={:?}",
                method,
                mask_signature(&request_args.url),
                mask_headers(&request_args.headers),
                status,
                &response
            );
            return Err(Error::Http(response.error_for_status().unwrap_err()));
        }

        // First parse the response as serde_json::Value
        let value: serde_json::Value = response.json().await.map_err(Error::Http)?;
        let ret_code = value.get("code").and_then(|v| v.as_i64()).unwrap_or(0);

        if ret_code != 0 {
            let err = ExchangeResponseError::from(value);
            log::error!(
                "ExchangeResponseError during async request: method={}, url={}, headers={:?}, status={}, error={}",
                method,
                mask_signature(&request_args.url),
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
