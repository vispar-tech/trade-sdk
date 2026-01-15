//! Main Bybit client class.

use crate::error::Result;
use crate::http::BybitHttpClient;

/// Bybit Trading API Client with all available methods.
pub struct BybitClient {
    http_client: BybitHttpClient,
}

impl BybitClient {
    /// Create a new Bybit client
    pub fn new(
        api_key: Option<String>,
        api_secret: Option<String>,
        testnet: bool,
        demo: bool,
        recv_window: u32,
        referral_id: Option<String>,
    ) -> Result<Self> {
        let http_client =
            BybitHttpClient::new(api_key, api_secret, testnet, demo, recv_window, referral_id);

        Ok(Self { http_client })
    }

    /// Check if this client uses a shared session
    pub fn uses_shared_session(&self) -> bool {
        self.http_client.uses_shared_session()
    }
}

// HttpClient is implemented via delegation to BybitHttpClient
#[async_trait::async_trait]
impl crate::traits::HttpClient for BybitClient {
    async fn get(
        &self,
        endpoint: &str,
        params: Option<&std::collections::HashMap<String, String>>,
        auth: bool,
    ) -> crate::error::Result<crate::types::GenericResponse> {
        self.http_client.get(endpoint, params, auth).await
    }

    async fn post(
        &self,
        endpoint: &str,
        params: Option<&std::collections::HashMap<String, String>>,
        auth: bool,
    ) -> crate::error::Result<crate::types::GenericResponse> {
        self.http_client.post(endpoint, params, auth).await
    }

    async fn put(
        &self,
        endpoint: &str,
        params: Option<&std::collections::HashMap<String, String>>,
        auth: bool,
    ) -> crate::error::Result<crate::types::GenericResponse> {
        self.http_client.put(endpoint, params, auth).await
    }

    async fn delete(
        &self,
        endpoint: &str,
        params: Option<&std::collections::HashMap<String, String>>,
        auth: bool,
    ) -> crate::error::Result<crate::types::GenericResponse> {
        self.http_client.delete(endpoint, params, auth).await
    }
}

// All API traits are implemented in their respective API modules

// Additional methods can be added here as needed
