//! Bybit Trading API Client with all available methods.
mod api;
mod http;
pub mod traits;
pub mod types;

use crate::error::Result;
use http::BybitHttpClient;
use linkme::distributed_slice;

#[distributed_slice]
pub static BYBIT_IMPLEMENTED: [&'static str];

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
            BybitHttpClient::new(api_key, api_secret, testnet, demo, recv_window, referral_id)?;
        Ok(Self { http_client })
    }
}

impl std::ops::Deref for BybitClient {
    type Target = BybitHttpClient;

    fn deref(&self) -> &Self::Target {
        &self.http_client
    }
}
