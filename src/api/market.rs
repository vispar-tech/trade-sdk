//! Market data API implementation.

use std::collections::HashMap;

use async_trait::async_trait;

use crate::{
    error::Result,
    traits::{HttpClient, MarketApi},
    types::{AllCategories, ApiResponse, InstrumentStatus, SymbolType},
};

/// Default implementation of MarketApi for BybitClient
#[async_trait]
impl<T: HttpClient + Send + Sync> MarketApi for T {
    async fn get_server_time(&self) -> Result<ApiResponse<serde_json::Value>> {
        let response = self.get("/v5/market/time", None, false).await?;
        Ok(response.into_api_response())
    }

    async fn get_kline(
        &self,
        symbol: &str,
        interval: &str,
        category: Option<&AllCategories>,
        start: Option<i64>,
        end: Option<i64>,
        limit: Option<i32>,
    ) -> Result<ApiResponse<serde_json::Value>> {
        let mut params = HashMap::new();
        params.insert("symbol".to_string(), symbol.to_string());
        params.insert("interval".to_string(), interval.to_string());

        if let Some(category) = category {
            params.insert("category".to_string(), category.to_string());
        }
        if let Some(start) = start {
            params.insert("start".to_string(), start.to_string());
        }
        if let Some(end) = end {
            params.insert("end".to_string(), end.to_string());
        }
        if let Some(limit) = limit {
            params.insert("limit".to_string(), limit.to_string());
        }

        let response = self.get("/v5/market/kline", Some(&params), false).await?;
        Ok(response.into_api_response())
    }

    async fn get_instruments_info(
        &self,
        category: AllCategories,
        symbol: Option<&str>,
        symbol_type: Option<&SymbolType>,
        status: Option<&InstrumentStatus>,
        base_coin: Option<&str>,
        limit: Option<i32>,
        cursor: Option<&str>,
    ) -> Result<ApiResponse<serde_json::Value>> {
        let mut params = HashMap::new();
        params.insert("category".to_string(), category.to_string());

        if let Some(symbol) = symbol {
            params.insert("symbol".to_string(), symbol.to_string());
        }
        if let Some(symbol_type) = symbol_type {
            params.insert("symbolType".to_string(), symbol_type.to_string());
        }
        if let Some(status) = status {
            params.insert("status".to_string(), status.to_string());
        }
        if let Some(base_coin) = base_coin {
            params.insert("baseCoin".to_string(), base_coin.to_string());
        }
        if let Some(limit) = limit {
            params.insert("limit".to_string(), limit.to_string());
        }
        if let Some(cursor) = cursor {
            params.insert("cursor".to_string(), cursor.to_string());
        }

        let response = self
            .get("/v5/market/instruments-info", Some(&params), false)
            .await?;
        Ok(response.into_api_response())
    }

    // TODO: Implement remaining methods
    async fn get_mark_price_kline(&self) -> Result<ApiResponse<serde_json::Value>> {
        todo!("get_mark_price_kline not implemented")
    }

    async fn get_index_price_kline(&self) -> Result<ApiResponse<serde_json::Value>> {
        todo!("get_index_price_kline not implemented")
    }

    async fn get_premium_index_price_kline(&self) -> Result<ApiResponse<serde_json::Value>> {
        todo!("get_premium_index_price_kline not implemented")
    }

    async fn get_orderbook(&self) -> Result<ApiResponse<serde_json::Value>> {
        todo!("get_orderbook not implemented")
    }

    async fn get_rpi_orderbook(&self) -> Result<ApiResponse<serde_json::Value>> {
        todo!("get_rpi_orderbook not implemented")
    }

    async fn get_tickers(&self) -> Result<ApiResponse<serde_json::Value>> {
        todo!("get_tickers not implemented")
    }

    async fn get_funding_rate_history(&self) -> Result<ApiResponse<serde_json::Value>> {
        todo!("get_funding_rate_history not implemented")
    }

    async fn get_recent_public_trades(&self) -> Result<ApiResponse<serde_json::Value>> {
        todo!("get_recent_public_trades not implemented")
    }

    async fn get_open_interest(&self) -> Result<ApiResponse<serde_json::Value>> {
        todo!("get_open_interest not implemented")
    }

    async fn get_historical_volatility(&self) -> Result<ApiResponse<serde_json::Value>> {
        todo!("get_historical_volatility not implemented")
    }

    async fn get_insurance_pool(&self) -> Result<ApiResponse<serde_json::Value>> {
        todo!("get_insurance_pool not implemented")
    }

    async fn get_risk_limit(&self) -> Result<ApiResponse<serde_json::Value>> {
        todo!("get_risk_limit not implemented")
    }

    async fn get_delivery_price(&self) -> Result<ApiResponse<serde_json::Value>> {
        todo!("get_delivery_price not implemented")
    }

    async fn get_new_delivery_price(&self) -> Result<ApiResponse<serde_json::Value>> {
        todo!("get_new_delivery_price not implemented")
    }

    async fn get_long_short_ratio(&self) -> Result<ApiResponse<serde_json::Value>> {
        todo!("get_long_short_ratio not implemented")
    }

    async fn get_index_price_components(&self) -> Result<ApiResponse<serde_json::Value>> {
        todo!("get_index_price_components not implemented")
    }

    async fn get_order_price_limit(&self) -> Result<ApiResponse<serde_json::Value>> {
        todo!("get_order_price_limit not implemented")
    }

    async fn get_adl_alert(&self) -> Result<ApiResponse<serde_json::Value>> {
        todo!("get_adl_alert not implemented")
    }

    async fn get_fee_group_structure(&self) -> Result<ApiResponse<serde_json::Value>> {
        todo!("get_fee_group_structure not implemented")
    }
}
