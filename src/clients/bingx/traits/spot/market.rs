use crate::{bingx::types::ApiResponse, error::Result};
use async_trait::async_trait;
use serde_json::Value;

#[async_trait]
pub trait MarketApi {
    /// Query tradable spot pairs whose symbol contains the given string.
    ///
    /// GET /openApi/spot/v1/common/symbols
    ///
    /// See: https://bingx-api.github.io/docs-v3/#/en/Spot/Market%20Data/Spot%20trading%20symbols
    async fn get_spot_symbols_like(
        &self,
        symbol: Option<&str>,
    ) -> Result<ApiResponse<Value>>;

    /// Retrieve Kline/Candlestick data for BingX Spot market.
    ///
    /// GET /openApi/spot/v2/market/kline
    ///
    /// See: https://bingx-api.github.io/docs-v3/#/en/Spot/Market%20Data/Kline%2FCandlestick%20Data
    ///
    /// - `symbol`: Trading pair symbol, e.g. "BTC-USDT".
    /// - `interval`: Candle interval, e.g. "1m", "5m", "1h", "1d", etc.
    /// - `start_time`: Start timestamp (ms), inclusive. (optional)
    /// - `end_time`: End timestamp (ms), inclusive. (optional)
    /// - `limit`: How many klines to return (default 500, max 1440). (optional)
    async fn get_spot_klines(
        &self,
        symbol: &str,
        interval: &str,
        start_time: Option<u64>,
        end_time: Option<u64>,
        limit: Option<u32>,
    ) -> Result<ApiResponse<Value>>;
}
