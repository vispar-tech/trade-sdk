use crate::bingx::traits::swap::TradeApi;
use crate::bingx::types::{
    ApiResponse, MarginMode, PlaceSwapOrderParams, PositionSide, QuoteCurrency, SwapOrderType,
};
use crate::bingx::BingxClient;
use crate::error::{Error, Result};
use crate::http::HttpClient;
use async_trait::async_trait;
use std::collections::HashMap;

use crate::bingx::BINGX_IMPLEMENTED;
use linkme::distributed_slice;

#[distributed_slice(BINGX_IMPLEMENTED)]
static PLACE_SWAP_ORDER: &str = "place_swap_order";

#[distributed_slice(BINGX_IMPLEMENTED)]
static CLOSE_SWAP_POSITION: &str = "close_swap_position";

#[distributed_slice(BINGX_IMPLEMENTED)]
static GET_SWAP_ORDER_HISTORY: &str = "get_swap_order_history";

#[distributed_slice(BINGX_IMPLEMENTED)]
static GET_SWAP_ORDER_DETAILS: &str = "get_swap_order_details";

#[distributed_slice(BINGX_IMPLEMENTED)]
static GET_SWAP_OPEN_ORDERS: &str = "get_swap_open_orders";

#[distributed_slice(BINGX_IMPLEMENTED)]
static CANCEL_SWAP_BATCH_ORDERS: &str = "cancel_swap_batch_orders";

#[distributed_slice(BINGX_IMPLEMENTED)]
static GET_SWAP_POSITION_HISTORY: &str = "get_swap_position_history";

#[distributed_slice(BINGX_IMPLEMENTED)]
static SET_SWAP_LEVERAGE: &str = "set_swap_leverage";

#[distributed_slice(BINGX_IMPLEMENTED)]
static SET_SWAP_POSITION_MODE: &str = "set_swap_position_mode";

#[distributed_slice(BINGX_IMPLEMENTED)]
static GET_SWAP_POSITION_MODE: &str = "get_swap_position_mode";

#[distributed_slice(BINGX_IMPLEMENTED)]
static GET_SWAP_LEVERAGE_AND_AVAILABLE_POSITIONS: &str =
    "get_swap_leverage_and_available_positions";

#[distributed_slice(BINGX_IMPLEMENTED)]
static CANCEL_ALL_SWAP_OPEN_ORDERS: &str = "cancel_all_swap_open_orders";

#[distributed_slice(BINGX_IMPLEMENTED)]
static CHANGE_SWAP_MARGIN_TYPE: &str = "change_swap_margin_type";

#[distributed_slice(BINGX_IMPLEMENTED)]
static GET_SWAP_MARGIN_TYPE: &str = "get_swap_margin_type";

#[async_trait]
impl TradeApi for BingxClient {
    async fn place_swap_order(
        &self,
        params: &PlaceSwapOrderParams,
    ) -> Result<ApiResponse<serde_json::Value>> {
        let json_value = serde_json::to_value(params)?;
        let mut order_data: HashMap<String, serde_json::Value> = HashMap::new();

        if let Some(obj) = json_value.as_object() {
            for (key, value) in obj {
                if !value.is_null() {
                    order_data.insert(key.clone(), value.clone());
                }
            }
        }

        let response = self
            .post("/openApi/swap/v2/trade/order", Some(&order_data), true)
            .await?;
        Ok(response.into_api_response())
    }

    async fn close_swap_position(
        &self,
        position_id: &str,
    ) -> Result<ApiResponse<serde_json::Value>> {
        let mut params: HashMap<String, serde_json::Value> = HashMap::new();
        params.insert(
            "positionId".to_string(),
            serde_json::Value::String(position_id.to_string()),
        );

        let response = self
            .post("/openApi/swap/v1/trade/closePosition", Some(&params), true)
            .await?;
        Ok(response.into_api_response())
    }

    async fn get_swap_order_history(
        &self,
        symbol: Option<&str>,
        currency: Option<QuoteCurrency>,
        order_id: Option<i64>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        limit: Option<u32>,
    ) -> Result<ApiResponse<serde_json::Value>> {
        let mut params: HashMap<String, serde_json::Value> = HashMap::new();

        if let Some(symbol) = symbol {
            params.insert(
                "symbol".to_string(),
                serde_json::Value::String(symbol.to_string()),
            );
        }
        if let Some(currency) = currency {
            params.insert(
                "currency".to_string(),
                serde_json::Value::String(currency.to_string()),
            );
        }
        if let Some(order_id) = order_id {
            params.insert("orderId".to_string(), serde_json::Value::from(order_id));
        }
        if let Some(start_time) = start_time {
            params.insert("startTime".to_string(), serde_json::Value::from(start_time));
        }
        if let Some(end_time) = end_time {
            params.insert("endTime".to_string(), serde_json::Value::from(end_time));
        }
        params.insert(
            "limit".to_string(),
            serde_json::Value::from(limit.unwrap_or(500)),
        );

        let response = self
            .get("/openApi/swap/v2/trade/allOrders", Some(&params), true)
            .await?;
        Ok(response.into_api_response())
    }

    async fn get_swap_order_details(
        &self,
        symbol: &str,
        order_id: Option<i64>,
        client_order_id: Option<&str>,
    ) -> Result<ApiResponse<serde_json::Value>> {
        let mut params: HashMap<String, serde_json::Value> = HashMap::new();
        params.insert(
            "symbol".to_string(),
            serde_json::Value::String(symbol.to_string()),
        );
        if let Some(order_id) = order_id {
            params.insert("orderId".to_string(), serde_json::Value::from(order_id));
        }
        if let Some(client_order_id) = client_order_id {
            params.insert(
                "clientOrderId".to_string(),
                serde_json::Value::String(client_order_id.to_string()),
            );
        }
        let response = self
            .get("/openApi/swap/v2/trade/order", Some(&params), true)
            .await?;
        Ok(response.into_api_response())
    }

    async fn get_swap_open_orders(
        &self,
        symbol: Option<&str>,
        order_type: Option<SwapOrderType>,
    ) -> Result<ApiResponse<serde_json::Value>> {
        let mut params: HashMap<String, serde_json::Value> = HashMap::new();
        if let Some(symbol) = symbol {
            params.insert(
                "symbol".to_string(),
                serde_json::Value::String(symbol.to_string()),
            );
        }
        if let Some(order_type) = order_type {
            params.insert(
                "type".to_string(),
                serde_json::Value::String(order_type.to_string()),
            );
        }
        let response = self
            .get("/openApi/swap/v2/trade/openOrders", Some(&params), true)
            .await?;
        Ok(response.into_api_response())
    }

    async fn cancel_swap_batch_orders(
        &self,
        symbol: &str,
        order_id_list: Option<&[i64]>,
        client_order_id_list: Option<&[&str]>,
    ) -> Result<ApiResponse<serde_json::Value>> {
        if order_id_list.is_none() && client_order_id_list.is_none() {
            return Err(Error::Validation(
                "At least one of order_id_list or client_order_id_list must be provided."
                    .to_string(),
            ));
        }

        let mut params: HashMap<String, serde_json::Value> = HashMap::new();
        params.insert(
            "symbol".to_string(),
            serde_json::Value::String(symbol.to_string()),
        );
        if let Some(order_id_list) = order_id_list {
            params.insert(
                "orderIdList".to_string(),
                serde_json::Value::Array(
                    order_id_list
                        .iter()
                        .map(|id| serde_json::Value::from(*id))
                        .collect(),
                ),
            );
        }
        if let Some(client_order_id_list) = client_order_id_list {
            params.insert(
                "clientOrderIdList".to_string(),
                serde_json::Value::Array(
                    client_order_id_list
                        .iter()
                        .map(|id| serde_json::Value::String(id.to_string()))
                        .collect(),
                ),
            );
        }

        let response = self
            .delete("/openApi/swap/v2/trade/batchOrders", Some(&params), true)
            .await?;
        Ok(response.into_api_response())
    }

    async fn get_swap_position_history(
        &self,
        symbol: &str,
        currency: Option<QuoteCurrency>,
        position_id: Option<i64>,
        start_ts: Option<i64>,
        end_ts: Option<i64>,
        page_index: Option<i32>,
        page_size: Option<i32>,
    ) -> Result<ApiResponse<serde_json::Value>> {
        let mut params: std::collections::HashMap<String, serde_json::Value> =
            std::collections::HashMap::new();
        params.insert(
            "symbol".to_string(),
            serde_json::Value::String(symbol.to_string()),
        );
        if let Some(currency) = currency {
            params.insert(
                "currency".to_string(),
                serde_json::Value::String(currency.to_string()),
            );
        }
        if let Some(position_id) = position_id {
            params.insert(
                "positionId".to_string(),
                serde_json::Value::from(position_id),
            );
        }
        if let Some(start_ts) = start_ts {
            params.insert("startTs".to_string(), serde_json::Value::from(start_ts));
        }
        if let Some(end_ts) = end_ts {
            params.insert("endTs".to_string(), serde_json::Value::from(end_ts));
        }
        if let Some(page_index) = page_index {
            params.insert("pageIndex".to_string(), serde_json::Value::from(page_index));
        }
        if let Some(page_size) = page_size {
            params.insert("pageSize".to_string(), serde_json::Value::from(page_size));
        }

        let response = self
            .get(
                "/openApi/swap/v1/trade/positionHistory",
                Some(&params),
                true,
            )
            .await?;
        Ok(response.into_api_response())
    }

    async fn set_swap_leverage(
        &self,
        symbol: &str,
        side: PositionSide,
        leverage: i32,
    ) -> Result<ApiResponse<serde_json::Value>> {
        let mut params: std::collections::HashMap<String, serde_json::Value> =
            std::collections::HashMap::new();
        params.insert(
            "symbol".to_string(),
            serde_json::Value::String(symbol.to_string()),
        );
        params.insert(
            "side".to_string(),
            serde_json::Value::String(side.to_string()),
        );
        params.insert("leverage".to_string(), serde_json::Value::from(leverage));

        let response = self
            .post("/openApi/swap/v2/trade/leverage", Some(&params), true)
            .await?;
        Ok(response.into_api_response())
    }

    async fn set_swap_position_mode(
        &self,
        dual_side_position: bool,
    ) -> Result<ApiResponse<serde_json::Value>> {
        let mut params: std::collections::HashMap<String, serde_json::Value> =
            std::collections::HashMap::new();
        params.insert(
            "dualSidePosition".to_string(),
            serde_json::Value::String(dual_side_position.to_string().to_lowercase()),
        );
        let response = self
            .post("/openApi/swap/v1/positionSide/dual", Some(&params), true)
            .await?;
        Ok(response.into_api_response())
    }

    async fn get_swap_position_mode(&self) -> Result<ApiResponse<serde_json::Value>> {
        let response = self
            .get("/openApi/swap/v1/positionSide/dual", None, true)
            .await?;
        Ok(response.into_api_response())
    }

    async fn get_swap_leverage_and_available_positions(
        &self,
        symbol: &str,
    ) -> Result<ApiResponse<serde_json::Value>> {
        let mut params: std::collections::HashMap<String, serde_json::Value> =
            std::collections::HashMap::new();
        params.insert(
            "symbol".to_string(),
            serde_json::Value::String(symbol.to_string()),
        );
        let response = self
            .get("/openApi/swap/v2/trade/leverage", Some(&params), true)
            .await?;
        Ok(response.into_api_response())
    }

    async fn cancel_all_swap_open_orders(
        &self,
        symbol: Option<&str>,
        order_type: Option<SwapOrderType>,
    ) -> Result<ApiResponse<serde_json::Value>> {
        let mut params: std::collections::HashMap<String, serde_json::Value> =
            std::collections::HashMap::new();
        if let Some(s) = symbol {
            params.insert(
                "symbol".to_string(),
                serde_json::Value::String(s.to_string()),
            );
        }
        if let Some(ot) = order_type {
            params.insert(
                "type".to_string(),
                serde_json::Value::String(ot.to_string()),
            );
        }
        let response = self
            .delete("/openApi/swap/v2/trade/allOpenOrders", Some(&params), true)
            .await?;
        Ok(response.into_api_response())
    }

    async fn change_swap_margin_type(
        &self,
        symbol: &str,
        margin_type: MarginMode,
    ) -> Result<ApiResponse<serde_json::Value>> {
        let mut params: std::collections::HashMap<String, serde_json::Value> =
            std::collections::HashMap::new();
        params.insert(
            "symbol".to_string(),
            serde_json::Value::String(symbol.to_string()),
        );
        params.insert(
            "marginType".to_string(),
            serde_json::Value::String(margin_type.to_string()),
        );
        let response = self
            .post("/openApi/swap/v2/trade/marginType", Some(&params), true)
            .await?;
        Ok(response.into_api_response())
    }

    async fn get_swap_margin_type(
        &self,
        symbol: &str,
    ) -> Result<ApiResponse<serde_json::Value>> {
        let mut params: std::collections::HashMap<String, serde_json::Value> =
            std::collections::HashMap::new();
        params.insert(
            "symbol".to_string(),
            serde_json::Value::String(symbol.to_string()),
        );
        let response = self
            .get("/openApi/swap/v2/trade/marginType", Some(&params), true)
            .await?;
        Ok(response.into_api_response())
    }
}
