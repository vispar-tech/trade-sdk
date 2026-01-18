use crate::bingx::types::{ApiResponse, SpotOrderStatus, SpotOrderType};
use crate::error::Result;

#[async_trait::async_trait]
pub trait TradeApi {
    /// Retrieve the order history for BingX spot trading.
    ///
    /// Endpoint: GET /openApi/spot/v1/trade/historyOrders
    /// Docs: https://bingx-api.github.io/docs-v3/#/en/Spot/Trades%20Endpoints/Query%20Order%20history
    ///
    /// - If order_id is set, takes precedence; ignores times.
    /// - With start_time/end_time, order_id not required.
    /// - page_index * page_size must not exceed 10,000.
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
    ) -> Result<ApiResponse<serde_json::Value>>;


    /// Query order details for BingX spot trading.
    ///
    /// Endpoint: GET /openApi/spot/v1/trade/query
    /// Docs: https://bingx-api.github.io/docs-v3/#/en/Spot/Trades%20Endpoints/Query%20Order%20details
    ///
    /// - Must provide either `order_id` or `client_order_id`.
    /// - Master and sub accounts supported.
    /// - UID Rate Limit: 10/second.
    async fn get_spot_order_details(
        &self,
        symbol: &str,
        order_id: Option<i64>,
        client_order_id: Option<&str>,
    ) -> Result<ApiResponse<serde_json::Value>>;

    /// Query current open (pending) orders for BingX spot trading.
    ///
    /// Endpoint: GET /openApi/spot/v1/trade/openOrders
    /// Docs: https://bingx-api.github.io/docs-v3/#/en/Spot/Trades%20Endpoints/Current%20Open%20Orders
    ///
    /// - `symbol`: Trading pair, e.g., "BTC-USDT". Query all pending orders when None.
    /// - UID Rate Limit: 10/second.
    /// - Signature is required.
    /// - Master and sub accounts supported.
    async fn get_spot_open_orders(
        &self,
        symbol: Option<&str>,
    ) -> Result<ApiResponse<serde_json::Value>>;

    /// Cancel multiple spot orders in a batch.
    ///
    /// Endpoint: POST /openApi/spot/v1/trade/cancelOrders
    /// Docs: https://bingx-api.github.io/docs-v3/#/en/Spot/Trades%20Endpoints/Cancel%20multiple%20orders
    ///
    /// Parameters:
    /// - symbol: Trading pair (required)
    /// - order_ids: List of order IDs (optional)
    /// - client_order_ids: List of client order IDs (optional)
    /// - process: 0 or 1 (optional)
    ///
    /// Notes:
    /// - At least one of order_ids or client_order_ids must be provided.
    /// - UID Rate Limit: 2/second.
    /// - Signature is required.
    /// - Master and sub accounts supported.
    async fn cancel_spot_batch_orders(
        &self,
        symbol: &str,
        order_ids: Option<&[&str]>,
        client_order_ids: Option<&[&str]>,
        process: Option<i32>,
    ) -> Result<ApiResponse<serde_json::Value>>;

    /// Query transaction (trade) details for BingX spot orders.
    ///
    /// Endpoint: GET /openApi/spot/v1/trade/myTrades
    /// Docs: https://bingx-api.github.io/docs-v3/#/en/Spot/Trades%20Endpoints/Query%20transaction%20details
    ///
    /// - `symbol`: Trading pair, e.g. "BTC-USDT" (required, UPPERCASE)
    /// - `order_id`: Order ID (optional)
    /// - `start_time`: Start timestamp in milliseconds (optional)
    /// - `end_time`: End timestamp in milliseconds (optional)
    /// - `from_id`: Starting trade ID; by default retrieves the latest trade (optional)
    /// - `limit`: Number of returned results (optional, default 500, max 1000)
    ///
    /// Notes:
    /// - Can only check data within the past 7 days.
    /// - If start_time/end_time not filled or invalid, past 24 hours returned by default.
    /// - Max returns limited to 500 (default); maximum 1000 per request.
    /// - Returns a list sorted by 'time' field, from smallest to largest.
    /// - UID Rate Limit: 5/second.
    /// - Signature is required.
    /// - Master and sub accounts supported.
    async fn get_spot_trade_details(
        &self,
        symbol: &str,
        order_id: Option<i64>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        from_id: Option<i64>,
        limit: Option<i32>,
    ) -> Result<ApiResponse<serde_json::Value>>;

    /// Cancel all open spot orders on a symbol (or all symbols if not specified).
    ///
    /// Endpoint: POST /openApi/spot/v1/trade/cancelOpenOrders
    /// Docs: https://bingx-api.github.io/docs-v3/#/en/Spot/Trades%20Endpoints/Cancel%20all%20Open%20Orders%20on%20a%20Symbol
    ///
    /// Parameters:
    /// - symbol: Trading pair, e.g. "BTC-USDT" (optional). If not filled, cancel all orders.
    ///
    /// Returns:
    /// - ApiResponse<serde_json::Value>: API response.
    ///
    /// Notes:
    /// - UID Rate Limit: 2/second.
    /// - Signature is required.
    /// - Applicable to Master and Sub Accounts.
    async fn cancel_all_spot_open_orders(
        &self,
        symbol: Option<&str>,
    ) -> Result<ApiResponse<serde_json::Value>>;

}