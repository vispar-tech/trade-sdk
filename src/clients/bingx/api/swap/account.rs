use crate::bingx::traits::swap::AccountApi;
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
static GET_SPOT_ACCOUNT_ASSETS: &str = "get_spot_account_assets";

#[distributed_slice(BINGX_IMPLEMENTED)]
static GET_ACCOUNT_ASSET_OVERVIEW: &str = "get_account_asset_overview";

#[async_trait]
impl AccountApi for BingxClient {
    async fn get_swap_positions(
        &self,
        symbol: Option<&str>,
    ) -> Result<ApiResponse<serde_json::Value>> {
        let mut params: HashMap<String, Value> = HashMap::new();
        if let Some(symbol) = symbol {
            params.insert("symbol".to_string(), Value::String(symbol.to_string()));
        }
        let response = self
            .get("/openApi/swap/v2/user/positions", Some(&params), true)
            .await?;
        Ok(response.into_api_response())
    }

    async fn get_swap_account_balance(&self) -> Result<ApiResponse<serde_json::Value>> {
        let response = self
            .get("/openApi/swap/v3/user/balance", None, true)
            .await?;
        Ok(response.into_api_response())
    }
}
