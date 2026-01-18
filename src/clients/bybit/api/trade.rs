//! Trading API implementation.

use std::collections::HashMap;

use async_trait::async_trait;
use linkme::distributed_slice;
use serde_json::Value;

use crate::bybit::traits::TradeApi;
use crate::bybit::types::{
    AllCategories, ApiResponse, CancelOrderFilter, CancelOrderParams, GetOrderHistoryParams,
    OrderFilter, PlaceOrderParams,
};
use crate::bybit::BybitClient;
use crate::error::{Error, Result};
use crate::http::HttpClient;

use crate::bybit::BYBIT_IMPLEMENTED;

#[distributed_slice(BYBIT_IMPLEMENTED)]
pub static PLACE_ORDER: &'static str = "place_order";

#[distributed_slice(BYBIT_IMPLEMENTED)]
pub static CANCEL_ORDER: &'static str = "cancel_order";

#[distributed_slice(BYBIT_IMPLEMENTED)]
pub static GET_OPEN_AND_CLOSED_ORDERS: &'static str = "get_open_and_closed_orders";

#[distributed_slice(BYBIT_IMPLEMENTED)]
pub static CANCEL_ALL_ORDERS: &'static str = "cancel_all_orders";

#[distributed_slice(BYBIT_IMPLEMENTED)]
pub static GET_ORDER_HISTORY: &'static str = "get_order_history";

#[distributed_slice(BYBIT_IMPLEMENTED)]
pub static BATCH_PLACE_ORDER: &'static str = "batch_place_order";

#[distributed_slice(BYBIT_IMPLEMENTED)]
pub static BATCH_CANCEL_ORDER: &'static str = "batch_cancel_order";

/// Default implementation of TradeApi for BybitClient
#[async_trait]
impl TradeApi for BybitClient {
    async fn place_order(
        &self,
        category: AllCategories,
        params: &PlaceOrderParams,
    ) -> Result<ApiResponse<Value>> {
        let mut api_params: HashMap<String, Value> = HashMap::new();

        // Add category as a string value (Value)
        api_params.insert("category".to_string(), Value::String(category.to_string()));

        // Add all non-null parameters from PlaceOrderParams
        let json_value = serde_json::to_value(params)?;
        if let Some(obj) = json_value.as_object() {
            for (key, value) in obj {
                if !value.is_null() {
                    api_params.insert(key.clone(), value.clone());
                }
            }
        }

        let response = self
            .post("/v5/order/create", Some(&api_params), true)
            .await?;
        Ok(response.into_api_response())
    }

    async fn cancel_order(
        &self,
        category: AllCategories,
        symbol: &str,
        order_id: Option<&str>,
        order_link_id: Option<&str>,
        order_filter: Option<&CancelOrderFilter>,
    ) -> Result<ApiResponse<Value>> {
        if order_id.is_none() && order_link_id.is_none() {
            return Err(Error::Validation(
                "Either order_id or order_link_id must be provided".to_string(),
            ));
        }

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("category".to_string(), Value::String(category.to_string()));
        api_params.insert("symbol".to_string(), Value::String(symbol.to_string()));

        if let Some(order_id) = order_id {
            api_params.insert("orderId".to_string(), Value::String(order_id.to_string()));
        }
        if let Some(order_link_id) = order_link_id {
            api_params.insert(
                "orderLinkId".to_string(),
                Value::String(order_link_id.to_string()),
            );
        }
        if let Some(order_filter) = order_filter {
            api_params.insert(
                "orderFilter".to_string(),
                Value::String(order_filter.to_string()),
            );
        }

        let response = self
            .post("/v5/order/cancel", Some(&api_params), true)
            .await?;
        Ok(response.into_api_response())
    }

    async fn get_open_and_closed_orders(
        &self,
        category: AllCategories,
        symbol: Option<&str>,
        base_coin: Option<&str>,
        settle_coin: Option<&str>,
        order_id: Option<&str>,
        order_link_id: Option<&str>,
        open_only: Option<bool>,
        order_filter: Option<&OrderFilter>,
        limit: Option<i32>,
        cursor: Option<&str>,
    ) -> Result<ApiResponse<Value>> {
        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("category".to_string(), Value::String(category.to_string()));

        if let Some(symbol) = symbol {
            api_params.insert("symbol".to_string(), Value::String(symbol.to_string()));
        }
        if let Some(base_coin) = base_coin {
            api_params.insert("baseCoin".to_string(), Value::String(base_coin.to_string()));
        }
        if let Some(settle_coin) = settle_coin {
            api_params.insert(
                "settleCoin".to_string(),
                Value::String(settle_coin.to_string()),
            );
        }
        if let Some(order_id) = order_id {
            api_params.insert("orderId".to_string(), Value::String(order_id.to_string()));
        }
        if let Some(order_link_id) = order_link_id {
            api_params.insert(
                "orderLinkId".to_string(),
                Value::String(order_link_id.to_string()),
            );
        }
        if let Some(open_only) = open_only {
            api_params.insert("openOnly".to_string(), Value::Bool(open_only));
        }
        if let Some(order_filter) = order_filter {
            api_params.insert(
                "orderFilter".to_string(),
                Value::String(order_filter.to_string()),
            );
        }
        if let Some(limit) = limit {
            api_params.insert("limit".to_string(), Value::Number(limit.into()));
        }
        if let Some(cursor) = cursor {
            api_params.insert("cursor".to_string(), Value::String(cursor.to_string()));
        }

        let response = self
            .get("/v5/order/realtime", Some(&api_params), true)
            .await?;
        Ok(response.into_api_response())
    }

    async fn cancel_all_orders(
        &self,
        category: AllCategories,
        symbol: Option<&str>,
        base_coin: Option<&str>,
        settle_coin: Option<&str>,
        order_filter: Option<&OrderFilter>,
        stop_order_type: Option<&str>,
    ) -> Result<ApiResponse<Value>> {
        // Validation for required parameters
        if matches!(category, AllCategories::Linear | AllCategories::Inverse)
            && symbol.is_none()
            && base_coin.is_none()
            && settle_coin.is_none()
        {
            return Err(Error::Validation(
                "For linear/inverse, provide symbol or base_coin or settle_coin".to_string(),
            ));
        }

        if matches!(category, AllCategories::Option)
            && symbol.is_none()
            && base_coin.is_none()
            && settle_coin.is_none()
        {
            return Err(Error::Validation(
                "For option, provide symbol or base_coin or settle_coin".to_string(),
            ));
        }

        let mut api_params = HashMap::new();
        api_params.insert("category".to_string(), Value::String(category.to_string()));

        if let Some(symbol) = symbol {
            api_params.insert("symbol".to_string(), Value::String(symbol.to_string()));
        }
        if let Some(base_coin) = base_coin {
            api_params.insert("baseCoin".to_string(), Value::String(base_coin.to_string()));
        }
        if let Some(settle_coin) = settle_coin {
            api_params.insert(
                "settleCoin".to_string(),
                Value::String(settle_coin.to_string()),
            );
        }
        if let Some(order_filter) = order_filter {
            api_params.insert(
                "orderFilter".to_string(),
                Value::String(order_filter.to_string()),
            );
        }
        if let Some(stop_order_type) = stop_order_type {
            api_params.insert(
                "stopOrderType".to_string(),
                Value::String(stop_order_type.to_string()),
            );
        }

        let response = self
            .post("/v5/order/cancel-all", Some(&api_params), true)
            .await?;
        Ok(response.into_api_response())
    }

    async fn get_order_history(
        &self,
        category: AllCategories,
        params: Option<&GetOrderHistoryParams>,
    ) -> Result<ApiResponse<Value>> {
        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("category".to_string(), Value::String(category.to_string()));

        if let Some(params) = params {
            let json_value = serde_json::to_value(params)?;
            if let Some(obj) = json_value.as_object() {
                for (key, value) in obj {
                    if !value.is_null() {
                        api_params.insert(key.clone(), value.clone());
                    }
                }
            }
        }

        let response = self
            .get("/v5/order/history", Some(&api_params), true)
            .await?;
        Ok(response.into_api_response())
    }

    async fn batch_place_order(
        &self,
        category: AllCategories,
        orders: &[PlaceOrderParams],
    ) -> Result<ApiResponse<Value>> {
        let mut request_data = Vec::with_capacity(orders.len());
        for order in orders {
            request_data.push(serde_json::to_value(order)?);
        }

        let mut params: HashMap<String, Value> = HashMap::new();
        params.insert("category".to_string(), Value::String(category.to_string()));
        params.insert("request".to_string(), serde_json::to_value(&request_data)?);

        let response = self
            .post("/v5/order/create-batch", Some(&params), true)
            .await?;
        Ok(response.into_api_response())
    }

    async fn batch_cancel_order(
        &self,
        category: AllCategories,
        orders: &[CancelOrderParams],
    ) -> Result<ApiResponse<Value>> {
        // Validate that each order has either orderId or orderLinkId
        for (i, order) in orders.iter().enumerate() {
            if order.order_id.is_none() && order.order_link_id.is_none() {
                return Err(Error::Validation(format!(
                    "Order at index {} must have either order_id or order_link_id",
                    i
                )));
            }
        }

        let mut request_data = Vec::with_capacity(orders.len());
        for order in orders {
            let value = serde_json::to_value(order)?;
            request_data.push(value);
        }

        let mut params: HashMap<String, Value> = HashMap::new();
        params.insert("category".to_string(), Value::String(category.to_string()));
        params.insert("request".to_string(), serde_json::to_value(&request_data)?);

        let response = self
            .post("/v5/order/cancel-batch", Some(&params), true)
            .await?;
        Ok(response.into_api_response())
    }

    // TODO: Implement remaining methods
    async fn amend_order(&self) -> Result<ApiResponse<Value>> {
        todo!("amend_order not implemented")
    }

    async fn get_trade_history(&self) -> Result<ApiResponse<Value>> {
        todo!("get_trade_history not implemented")
    }

    async fn batch_amend_order(&self) -> Result<ApiResponse<Value>> {
        todo!("batch_amend_order not implemented")
    }

    async fn get_borrow_quota_spot(&self) -> Result<ApiResponse<Value>> {
        todo!("get_borrow_quota_spot not implemented")
    }

    async fn set_dcp(&self) -> Result<ApiResponse<Value>> {
        todo!("set_dcp not implemented")
    }

    async fn pre_check_order(&self) -> Result<ApiResponse<Value>> {
        todo!("pre_check_order not implemented")
    }
}
