use crate::bingx::traits::swap::MarketApi;
use crate::bingx::types::ApiResponse;
use crate::bingx::BingxClient;
use crate::error::Result;
use crate::http::HttpClient;
use async_trait::async_trait;
use serde_json::Value;
use std::collections::HashMap;
use linkme::distributed_slice;
use crate::bingx::BINGX_IMPLEMENTED;

#[distributed_slice(BINGX_IMPLEMENTED)]
static GET_SWAP_CONTRACTS: &str = "get_swap_contracts";

#[distributed_slice(BINGX_IMPLEMENTED)]
static GET_SWAP_KLINES: &str = "get_swap_klines";

#[async_trait]
impl MarketApi for BingxClient {
    async fn get_swap_contracts(
        &self,
        symbol: Option<&str>,
    ) -> Result<ApiResponse<serde_json::Value>> {
        let mut params: HashMap<String, Value> = HashMap::new();
        if let Some(symbol) = symbol {
            params.insert("symbol".to_string(), Value::String(symbol.to_string()));
        }
        let response = self
            .get("/openApi/swap/v2/quote/contracts", Some(&params), false)
            .await?;
        Ok(response.into_api_response())
    }

    async fn get_swap_klines(
        &self,
        symbol: &str,
        interval: &str,
        start_time: Option<i64>,
        end_time: Option<i64>,
        limit: Option<u32>,
    ) -> Result<ApiResponse<serde_json::Value>> {
        let mut params: HashMap<String, Value> = HashMap::new();
        params.insert("symbol".to_string(), Value::String(symbol.to_string()));
        params.insert("interval".to_string(), Value::String(interval.to_string()));
        if let Some(start_time) = start_time {
            params.insert("startTime".to_string(), Value::from(start_time));
        }
        if let Some(end_time) = end_time {
            params.insert("endTime".to_string(), Value::from(end_time));
        }
        if let Some(limit) = limit {
            params.insert("limit".to_string(), Value::from(limit));
        }

        let response = self
            .get("/openApi/swap/v3/quote/klines", Some(&params), false)
            .await?;
        Ok(response.into_api_response())
    }
}
