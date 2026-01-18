//! Bingx Trading API Client with all available methods.
mod api;
mod http;
pub mod traits;
pub mod types;

use crate::error::Result;
use http::BingxHttpClient;
use linkme::distributed_slice;

#[distributed_slice]
pub static BINGX_IMPLEMENTED: [&'static str];

/// Bybit Trading API Client with all available methods.
pub struct BingxClient {
    http_client: BingxHttpClient,
}

impl BingxClient {
    /// Create a new Bybit client
    pub fn new(
        api_key: Option<String>,
        api_secret: Option<String>,

        demo: bool,
        recv_window: u32,
    ) -> Result<Self> {
        let http_client = BingxHttpClient::new(api_key, api_secret, demo, recv_window)?;
        Ok(Self { http_client })
    }
}

impl std::ops::Deref for BingxClient {
    type Target = BingxHttpClient;

    fn deref(&self) -> &Self::Target {
        &self.http_client
    }
}
