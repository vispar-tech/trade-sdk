//! Position API implementation.

use std::collections::HashMap;

use async_trait::async_trait;
use serde_json::Value;

use crate::bybit::traits::PositionApi;
use crate::bybit::types::{AllCategories, ApiResponse, SetTradingStopParams};
use crate::bybit::BybitClient;
use crate::error::Error;
use crate::error::Result;
use crate::http::HttpClient;

use crate::bybit::BYBIT_IMPLEMENTED;
use linkme::distributed_slice;

#[distributed_slice(BYBIT_IMPLEMENTED)]
pub static GET_POSITION_INFO: &'static str = "get_position_info";

#[distributed_slice(BYBIT_IMPLEMENTED)]
pub static SET_LEVERAGE: &'static str = "set_leverage";

#[distributed_slice(BYBIT_IMPLEMENTED)]
pub static SWITCH_POSITION_MODE: &'static str = "switch_position_mode";

#[distributed_slice(BYBIT_IMPLEMENTED)]
pub static SET_TRADING_STOP: &'static str = "set_trading_stop";

#[distributed_slice(BYBIT_IMPLEMENTED)]
pub static GET_CLOSED_PNL: &'static str = "get_closed_pnl";

/// Default implementation of PositionApi for BybitClient
#[async_trait]
impl PositionApi for BybitClient {
    async fn get_position_info(
        &self,
        category: AllCategories,
        symbol: Option<&str>,
        base_coin: Option<&str>,
        settle_coin: Option<&str>,
        limit: Option<u32>,
        cursor: Option<&str>,
    ) -> Result<ApiResponse<Value>> {
        let mut params: HashMap<String, Value> = HashMap::new();
        params.insert("category".to_string(), Value::String(category.to_string()));

        if let Some(symbol) = symbol {
            params.insert("symbol".to_string(), Value::String(symbol.to_string()));
        }
        if let Some(base_coin) = base_coin {
            params.insert("baseCoin".to_string(), Value::String(base_coin.to_string()));
        }
        if let Some(settle_coin) = settle_coin {
            params.insert(
                "settleCoin".to_string(),
                Value::String(settle_coin.to_string()),
            );
        }
        if let Some(limit) = limit {
            params.insert("limit".to_string(), Value::Number(limit.into()));
        }
        if let Some(cursor) = cursor {
            params.insert("cursor".to_string(), Value::String(cursor.to_string()));
        }

        let response = self.get("/v5/position/list", Some(&params), true).await?;
        Ok(response.into_api_response())
    }

    async fn set_leverage(
        &self,
        category: AllCategories,
        symbol: &str,
        buy_leverage: u32,
        sell_leverage: u32,
    ) -> Result<ApiResponse<Value>> {
        let mut params: HashMap<String, Value> = HashMap::new();
        params.insert("category".to_string(), Value::String(category.to_string()));
        params.insert("symbol".to_string(), Value::String(symbol.to_string()));
        params.insert(
            "buyLeverage".to_string(),
            Value::Number(buy_leverage.into()),
        );
        params.insert(
            "sellLeverage".to_string(),
            Value::Number(sell_leverage.into()),
        );

        let response = self
            .post("/v5/position/set-leverage", Some(&params), true)
            .await?;
        Ok(response.into_api_response())
    }

    async fn switch_position_mode(
        &self,
        category: AllCategories,
        mode: u8,
        symbol: Option<&str>,
        coin: Option<&str>,
    ) -> Result<ApiResponse<Value>> {
        if symbol.is_none() && coin.is_none() {
            return Err(Error::Validation(
                "Either symbol or coin must be provided".to_string(),
            ));
        }

        let mut params: HashMap<String, Value> = HashMap::new();
        params.insert("category".to_string(), Value::String(category.to_string()));
        params.insert("mode".to_string(), Value::Number(mode.into()));

        if let Some(symbol) = symbol {
            params.insert("symbol".to_string(), Value::String(symbol.to_string()));
        }
        if let Some(coin) = coin {
            params.insert("coin".to_string(), Value::String(coin.to_string()));
        }

        let response = self
            .post("/v5/position/switch-mode", Some(&params), true)
            .await?;
        Ok(response.into_api_response())
    }

    async fn set_trading_stop(
        &self,
        category: AllCategories,
        params: &SetTradingStopParams,
    ) -> Result<ApiResponse<Value>> {
        let json_value = serde_json::to_value(params)?;
        let mut api_params: HashMap<String, Value> = HashMap::new();

        // Add category
        api_params.insert("category".to_string(), Value::String(category.to_string()));

        // Add all non-null parameters from SetTradingStopParams
        if let Some(obj) = json_value.as_object() {
            for (key, value) in obj {
                if !value.is_null() {
                    api_params.insert(key.clone(), value.clone());
                }
            }
        }

        let response = self
            .post("/v5/position/trading-stop", Some(&api_params), true)
            .await?;
        Ok(response.into_api_response())
    }

    // TODO: Implement remaining methods
    async fn set_auto_add_margin(&self) -> Result<ApiResponse<Value>> {
        todo!("set_auto_add_margin not implemented")
    }

    async fn add_or_reduce_margin(&self) -> Result<ApiResponse<Value>> {
        todo!("add_or_reduce_margin not implemented")
    }

    async fn get_closed_pnl(
        &self,
        category: AllCategories,
        symbol: Option<&str>,
        start_time: Option<u64>,
        end_time: Option<u64>,
        limit: Option<u32>,
        cursor: Option<&str>,
    ) -> Result<ApiResponse<Value>> {
        let mut params: HashMap<String, Value> = HashMap::new();
        params.insert("category".to_string(), Value::String(category.to_string()));

        if let Some(symbol) = symbol {
            params.insert("symbol".to_string(), Value::String(symbol.to_string()));
        }
        if let Some(start_time) = start_time {
            params.insert("startTime".to_string(), Value::Number(start_time.into()));
        }
        if let Some(end_time) = end_time {
            params.insert("endTime".to_string(), Value::Number(end_time.into()));
        }
        if let Some(limit) = limit {
            params.insert("limit".to_string(), Value::Number(limit.into()));
        }
        if let Some(cursor) = cursor {
            params.insert("cursor".to_string(), Value::String(cursor.to_string()));
        }

        let response = self
            .get("/v5/position/closed-pnl", Some(&params), true)
            .await?;
        Ok(response.into_api_response())
    }

    // TODO: Implement remaining methods
    async fn get_closed_options_positions(&self) -> Result<ApiResponse<Value>> {
        todo!("get_closed_options_positions not implemented")
    }

    async fn move_position(&self) -> Result<ApiResponse<Value>> {
        todo!("move_position not implemented")
    }

    async fn get_move_position_history(&self) -> Result<ApiResponse<Value>> {
        todo!("get_move_position_history not implemented")
    }

    async fn confirm_new_risk_limit(&self) -> Result<ApiResponse<Value>> {
        todo!("confirm_new_risk_limit not implemented")
    }
}
