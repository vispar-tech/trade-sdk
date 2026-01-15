//! Position API implementation.

use std::collections::HashMap;

use async_trait::async_trait;

use crate::{
    error::Result,
    traits::{HttpClient, PositionApi},
    types::{AllCategories, ApiResponse, SetTradingStopParams},
};

/// Default implementation of PositionApi for BybitClient
#[async_trait]
impl<T: HttpClient + Send + Sync> PositionApi for T {
    async fn get_position_info(
        &self,
        category: AllCategories,
        symbol: Option<&str>,
        base_coin: Option<&str>,
        settle_coin: Option<&str>,
        limit: Option<u32>,
        cursor: Option<&str>,
    ) -> Result<ApiResponse<serde_json::Value>> {
        let mut params = HashMap::new();
        params.insert("category".to_string(), category.to_string());

        if let Some(symbol) = symbol {
            params.insert("symbol".to_string(), symbol.to_string());
        }
        if let Some(base_coin) = base_coin {
            params.insert("baseCoin".to_string(), base_coin.to_string());
        }
        if let Some(settle_coin) = settle_coin {
            params.insert("settleCoin".to_string(), settle_coin.to_string());
        }
        if let Some(limit) = limit {
            params.insert("limit".to_string(), limit.to_string());
        }
        if let Some(cursor) = cursor {
            params.insert("cursor".to_string(), cursor.to_string());
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
    ) -> Result<ApiResponse<serde_json::Value>> {
        let mut params = HashMap::new();
        params.insert("category".to_string(), category.to_string());
        params.insert("symbol".to_string(), symbol.to_string());
        params.insert("buyLeverage".to_string(), buy_leverage.to_string());
        params.insert("sellLeverage".to_string(), sell_leverage.to_string());

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
    ) -> Result<ApiResponse<serde_json::Value>> {
        if symbol.is_none() && coin.is_none() {
            return Err(crate::error::Error::Config(
                "Either symbol or coin must be provided".to_string(),
            ));
        }

        let mut params = HashMap::new();
        params.insert("category".to_string(), category.to_string());
        params.insert("mode".to_string(), mode.to_string());

        if let Some(symbol) = symbol {
            params.insert("symbol".to_string(), symbol.to_string());
        }
        if let Some(coin) = coin {
            params.insert("coin".to_string(), coin.to_string());
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
    ) -> Result<ApiResponse<serde_json::Value>> {
        let json_value = serde_json::to_value(params)?;
        let mut api_params = HashMap::new();

        // Add category
        api_params.insert("category".to_string(), category.to_string());

        // Add all non-null parameters from SetTradingStopParams
        if let Some(obj) = json_value.as_object() {
            for (key, value) in obj {
                if !value.is_null() {
                    api_params.insert(key.clone(), value.to_string());
                }
            }
        }

        let response = self
            .post("/v5/position/trading-stop", Some(&api_params), true)
            .await?;
        Ok(response.into_api_response())
    }

    // TODO: Implement remaining methods
    async fn set_auto_add_margin(&self) -> Result<ApiResponse<serde_json::Value>> {
        todo!("set_auto_add_margin not implemented")
    }

    async fn add_or_reduce_margin(&self) -> Result<ApiResponse<serde_json::Value>> {
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
    ) -> Result<ApiResponse<serde_json::Value>> {
        let mut params = HashMap::new();
        params.insert("category".to_string(), category.to_string());

        if let Some(symbol) = symbol {
            params.insert("symbol".to_string(), symbol.to_string());
        }
        if let Some(start_time) = start_time {
            params.insert("startTime".to_string(), start_time.to_string());
        }
        if let Some(end_time) = end_time {
            params.insert("endTime".to_string(), end_time.to_string());
        }
        if let Some(limit) = limit {
            params.insert("limit".to_string(), limit.to_string());
        }
        if let Some(cursor) = cursor {
            params.insert("cursor".to_string(), cursor.to_string());
        }

        let response = self
            .get("/v5/position/closed-pnl", Some(&params), true)
            .await?;
        Ok(response.into_api_response())
    }

    // TODO: Implement remaining methods
    async fn get_closed_options_positions(&self) -> Result<ApiResponse<serde_json::Value>> {
        todo!("get_closed_options_positions not implemented")
    }

    async fn move_position(&self) -> Result<ApiResponse<serde_json::Value>> {
        todo!("move_position not implemented")
    }

    async fn get_move_position_history(&self) -> Result<ApiResponse<serde_json::Value>> {
        todo!("get_move_position_history not implemented")
    }

    async fn confirm_new_risk_limit(&self) -> Result<ApiResponse<serde_json::Value>> {
        todo!("confirm_new_risk_limit not implemented")
    }
}
