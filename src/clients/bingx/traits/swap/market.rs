use crate::{bingx::types::ApiResponse, error::Result};

use async_trait::async_trait;

/// Trait for BingX Swap Market API methods.
///
/// This trait provides methods for retrieving market data and prices
/// for swap trading.
#[async_trait]
pub trait MarketApi {
    /// Retrieve contract information for BingX Perpetual Swaps.
    ///
    /// GET /openApi/swap/v2/quote/contracts
    ///
    /// [BingX API Documentation - Perp Futures symbols](https://bingx-api.github.io/docs-v3/#/en/Swap/Market%20Data/USDT-M%20Perp%20Futures%20symbols)
    ///
    /// # Arguments
    /// * `symbol` - Optionally filter by symbol (e.g., "BTC-USDT"). If `None`, returns all contracts.
    ///
    /// # Returns
    /// Returns an `ApiResponse` containing contract information.
    async fn get_swap_contracts(
        &self,
        symbol: Option<&str>,
    ) -> Result<ApiResponse<serde_json::Value>>;

    /// Retrieve Kline/Candlestick data for BingX Perpetual Swap contracts.
    ///
    /// GET /openApi/swap/v3/quote/klines
    ///
    /// [BingX API Documentation - Kline/Candlestick Data](https://bingx-api.github.io/docs-v3/#/en/Swap/Market%20Data/Kline%2FCandlestick%20Data)
    ///
    /// # Arguments
    /// * `symbol` - Trading pair symbol (e.g., "BTC-USDT"), must contain a hyphen.
    /// * `interval` - Kline interval (e.g. "1m", "5m", "1h", "1d").
    /// * `start_time` - Optional start timestamp (ms), inclusive.
    /// * `end_time` - Optional end timestamp (ms), inclusive.
    /// * `limit` - Optional: How many klines to return (default 500, max 1440).
    ///
    /// # Returns
    /// Returns an `ApiResponse` containing candlestick/kline data.
    async fn get_swap_klines(
        &self,
        symbol: &str,
        interval: &str,
        start_time: Option<i64>,
        end_time: Option<i64>,
        limit: Option<u32>,
    ) -> Result<ApiResponse<serde_json::Value>>;
}
