use std::collections::HashMap;

use async_trait::async_trait;

use crate::bingx::traits::spot::TradeApi;
use crate::bingx::types::{ApiResponse, SpotOrderStatus, SpotOrderType};
use crate::bingx::BingxClient;
use crate::bingx::BINGX_IMPLEMENTED;
use crate::error::{Error, Result};
use crate::http::HttpClient;
use linkme::distributed_slice;

#[distributed_slice(BINGX_IMPLEMENTED)]
static GET_SPOT_ORDER_HISTORY: &str = "get_spot_order_history";

#[distributed_slice(BINGX_IMPLEMENTED)]
static GET_SPOT_ORDER_DETAILS: &str = "get_spot_order_details";

#[distributed_slice(BINGX_IMPLEMENTED)]
static GET_SPOT_OPEN_ORDERS: &str = "get_spot_open_orders";

#[distributed_slice(BINGX_IMPLEMENTED)]
static CANCEL_SPOT_BATCH_ORDERS: &str = "cancel_spot_batch_orders";

#[distributed_slice(BINGX_IMPLEMENTED)]
static GET_SPOT_TRADE_DETAILS: &str = "get_spot_trade_details";

#[distributed_slice(BINGX_IMPLEMENTED)]
static CANCEL_ALL_SPOT_OPEN_ORDERS: &str = "cancel_all_spot_open_orders";

#[async_trait]
impl TradeApi for BingxClient {
    async fn get_spot_order_history(
        &self,
        symbol: Option<&str>,
        order_id: Option<i64>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        page_index: Option<i64>,
        page_size: Option<i64>,
        status: Option<SpotOrderStatus>,
        order_type: Option<SpotOrderType>,
    ) -> Result<ApiResponse<serde_json::Value>> {
        let mut params: HashMap<String, serde_json::Value> = HashMap::new();

        if let Some(symbol) = symbol {
            params.insert(
                "symbol".to_string(),
                serde_json::Value::String(symbol.to_owned()),
            );
        }
        if let Some(order_id) = order_id {
            params.insert(
                "orderId".to_string(),
                serde_json::Value::Number(order_id.into()),
            );
        }
        if let Some(start_time) = start_time {
            params.insert(
                "startTime".to_string(),
                serde_json::Value::Number(start_time.into()),
            );
        }
        if let Some(end_time) = end_time {
            params.insert(
                "endTime".to_string(),
                serde_json::Value::Number(end_time.into()),
            );
        }
        if let Some(page_index) = page_index {
            params.insert(
                "pageIndex".to_string(),
                serde_json::Value::Number(page_index.into()),
            );
        }
        if let Some(page_size) = page_size {
            params.insert(
                "pageSize".to_string(),
                serde_json::Value::Number(page_size.into()),
            );
        }
        if let Some(status) = status {
            params.insert(
                "status".to_string(),
                serde_json::Value::String(status.to_string()),
            );
        }
        if let Some(order_type) = order_type {
            params.insert(
                "type".to_string(),
                serde_json::Value::String(order_type.to_string()),
            );
        }

        let response = self
            .get("/openApi/spot/v1/trade/historyOrders", Some(&params), true)
            .await?;
        Ok(response.into_api_response())
    }

    async fn get_spot_order_details(
        &self,
        symbol: &str,
        order_id: Option<i64>,
        client_order_id: Option<&str>,
    ) -> Result<ApiResponse<serde_json::Value>> {
        let mut params: std::collections::HashMap<String, serde_json::Value> =
            std::collections::HashMap::new();
        params.insert(
            "symbol".to_string(),
            serde_json::Value::String(symbol.to_owned()),
        );
        if let Some(order_id) = order_id {
            params.insert(
                "orderId".to_string(),
                serde_json::Value::Number(order_id.into()),
            );
        }
        if let Some(client_order_id) = client_order_id {
            params.insert(
                "clientOrderID".to_string(),
                serde_json::Value::String(client_order_id.to_owned()),
            );
        }

        let response = self
            .get("/openApi/spot/v1/trade/query", Some(&params), true)
            .await?;
        Ok(response.into_api_response())
    }

    async fn get_spot_open_orders(
        &self,
        symbol: Option<&str>,
    ) -> Result<ApiResponse<serde_json::Value>> {
        let mut params: std::collections::HashMap<String, serde_json::Value> =
            std::collections::HashMap::new();
        if let Some(symbol) = symbol {
            params.insert(
                "symbol".to_string(),
                serde_json::Value::String(symbol.to_owned()),
            );
        }
        let response = self
            .get("/openApi/spot/v1/trade/openOrders", Some(&params), true)
            .await?;
        Ok(response.into_api_response())
    }

    async fn cancel_spot_batch_orders(
        &self,
        symbol: &str,
        order_ids: Option<&[&str]>,
        client_order_ids: Option<&[&str]>,
        process: Option<i32>,
    ) -> Result<ApiResponse<serde_json::Value>> {
        if order_ids.is_none() && client_order_ids.is_none() {
            return Err(Error::Validation(
                "At least one of order_ids or client_order_ids must be provided.".to_string(),
            ));
        }
        let mut params: std::collections::HashMap<String, serde_json::Value> =
            std::collections::HashMap::new();
        params.insert(
            "symbol".to_string(),
            serde_json::Value::String(symbol.to_owned()),
        );
        if let Some(order_ids) = order_ids {
            params.insert(
                "orderIds".to_string(),
                serde_json::Value::String(order_ids.join(",")),
            );
        }
        if let Some(client_order_ids) = client_order_ids {
            params.insert(
                "clientOrderIDs".to_string(),
                serde_json::Value::String(client_order_ids.join(",")),
            );
        }
        if let Some(process) = process {
            params.insert(
                "process".to_string(),
                serde_json::Value::Number(process.into()),
            );
        }
        let response = self
            .post("/openApi/spot/v1/trade/cancelOrders", Some(&params), true)
            .await?;
        Ok(response.into_api_response())
    }

    async fn get_spot_trade_details(
        &self,
        symbol: &str,
        order_id: Option<i64>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        from_id: Option<i64>,
        limit: Option<i32>,
    ) -> Result<ApiResponse<serde_json::Value>> {
        let mut params: std::collections::HashMap<String, serde_json::Value> =
            std::collections::HashMap::new();
        params.insert(
            "symbol".to_string(),
            serde_json::Value::String(symbol.to_owned()),
        );
        params.insert(
            "limit".to_string(),
            serde_json::Value::Number(serde_json::Number::from(limit.unwrap_or(500))),
        );
        if let Some(order_id) = order_id {
            params.insert(
                "orderId".to_string(),
                serde_json::Value::Number(order_id.into()),
            );
        }
        if let Some(start_time) = start_time {
            params.insert(
                "startTime".to_string(),
                serde_json::Value::Number(start_time.into()),
            );
        }
        if let Some(end_time) = end_time {
            params.insert(
                "endTime".to_string(),
                serde_json::Value::Number(end_time.into()),
            );
        }
        if let Some(from_id) = from_id {
            params.insert(
                "fromId".to_string(),
                serde_json::Value::Number(from_id.into()),
            );
        }
        let response = self
            .get("/openApi/spot/v1/trade/myTrades", Some(&params), true)
            .await?;
        Ok(response.into_api_response())
    }

    async fn cancel_all_spot_open_orders(
        &self,
        symbol: Option<&str>,
    ) -> Result<ApiResponse<serde_json::Value>> {
        let mut params: std::collections::HashMap<String, serde_json::Value> =
            std::collections::HashMap::new();
        if let Some(symbol) = symbol {
            params.insert(
                "symbol".to_string(),
                serde_json::Value::String(symbol.to_owned()),
            );
        }
        let response = self
            .post(
                "/openApi/spot/v1/trade/cancelOpenOrders",
                Some(&params),
                true,
            )
            .await?;
        Ok(response.into_api_response())
    }
}
