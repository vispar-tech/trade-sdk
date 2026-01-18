use crate::bingx::traits::spot::MarketApi;
use crate::bingx::types::ApiResponse;
use crate::bingx::BingxClient;
use crate::bingx::BINGX_IMPLEMENTED;
use crate::error::Result;
use crate::http::HttpClient;
use async_trait::async_trait;
use linkme::distributed_slice;
use serde_json::Value;
use std::collections::HashMap;

#[distributed_slice(BINGX_IMPLEMENTED)]
static GET_SPOT_SYMBOLS_LIKE: &str = "get_spot_symbols_like";

#[distributed_slice(BINGX_IMPLEMENTED)]
static GET_SPOT_KLINES: &str = "get_spot_klines";

#[async_trait]
impl MarketApi for BingxClient {
    async fn get_spot_symbols_like(
        &self,
        symbol: Option<&str>,
    ) -> Result<ApiResponse<serde_json::Value>> {
        let mut params: HashMap<String, Value> = HashMap::new();
        if let Some(sym) = symbol {
            params.insert("symbol".to_string(), Value::String(sym.to_string()));
        }
        let response = self
            .get("/openApi/spot/v1/common/symbols", Some(&params), false)
            .await?;
        Ok(response.into_api_response())
    }

    async fn get_spot_klines(
        &self,
        symbol: &str,
        interval: &str,
        start_time: Option<u64>,
        end_time: Option<u64>,
        limit: Option<u32>,
    ) -> Result<ApiResponse<serde_json::Value>> {
        let mut params: HashMap<String, Value> = HashMap::new();
        params.insert("symbol".to_string(), Value::String(symbol.to_string()));
        params.insert("interval".to_string(), Value::String(interval.to_string()));
        if let Some(s) = start_time {
            params.insert("startTime".to_string(), Value::from(s));
        }
        if let Some(e) = end_time {
            params.insert("endTime".to_string(), Value::from(e));
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), Value::from(l));
        }
        let response = self
            .get("/openApi/spot/v2/market/kline", Some(&params), false)
            .await?;
        Ok(response.into_api_response())
    }
}
