use crate::bingx::types::{
    ApiResponse, MarginMode, PlaceSwapOrderParams, PositionSide, QuoteCurrency, SwapOrderType,
};
use crate::error::Result;
use async_trait::async_trait;

/// Trading methods for BingX swap API client.
///
/// This trait provides methods for placing and managing trades in swap markets.
///
/// See also: <https://bingx-api.github.io/docs-v3/#/en/Swap/Trades%20Endpoints/Place%20Order>
#[async_trait]
pub trait TradeApi {
    /// Place a new swap order.
    ///
    /// Endpoint: POST /openApi/swap/v2/trade/order
    ///
    /// # Arguments
    /// * `params` - Parameters for the swap order, compliant with BingX API.
    ///
    /// # Returns
    /// * `ApiResponse<serde_json::Value>` - The API response.
    ///
    /// [BingX API Documentation - Place swap order](https://bingx-api.github.io/docs-v3/#/en/Swap/Trades%20Endpoints/Place%20Order)
    async fn place_swap_order(
        &self,
        params: &PlaceSwapOrderParams,
    ) -> Result<ApiResponse<serde_json::Value>>;

    /// Close a Perpetual Swap position by position ID.
    ///
    /// Endpoint: POST /openApi/swap/v1/trade/closePosition
    ///
    /// [BingX API Documentation - Close position by position ID](https://bingx-api.github.io/docs-v3/#/en/Swap/Trades%20Endpoints/Close%20position%20by%20position%20ID)
    ///
    /// # Arguments
    /// * `position_id` - The position ID to close.
    ///
    /// # Returns
    /// * `ApiResponse<serde_json::Value>` - The API response.
    async fn close_swap_position(
        &self,
        position_id: &str,
    ) -> Result<ApiResponse<serde_json::Value>>;

    /// Query swap order history (completed or canceled orders).
    ///
    /// Endpoint: GET /openApi/swap/v2/trade/allOrders
    ///
    /// [BingX API Documentation - Query Order history](https://bingx-api.github.io/docs-v3/#/en/Swap/Trades%20Endpoints/Query%20Order%20history)
    ///
    /// # Arguments
    /// * `symbol` - Optional trading pair symbol, e.g. "BTC-USDT". If not specified, returns all.
    /// * `currency` - Optional currency ("USDT" or "USDC").
    /// * `order_id` - Optional: Return orders after this orderId.
    /// * `start_time` - Optional: Start timestamp (ms).
    /// * `end_time` - Optional: End timestamp (ms).
    /// * `limit` - Optional: Number of results to return (default 500, max 1000).
    ///
    /// # Returns
    /// * `ApiResponse<serde_json::Value>` - API response with order history.
    async fn get_swap_order_history(
        &self,
        symbol: Option<&str>,
        currency: Option<QuoteCurrency>,
        order_id: Option<i64>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        limit: Option<u32>, // If None, use default 500
    ) -> Result<ApiResponse<serde_json::Value>>;

    /// Query swap order details (active, completed, or canceled orders).
    ///
    /// Endpoint: GET /openApi/swap/v2/trade/order
    ///
    /// [BingX API Documentation - Query Order details](https://bingx-api.github.io/docs-v3/#/en/Swap/Trades%20Endpoints/Query%20Order%20details)
    ///
    /// # Arguments
    /// * `symbol` - Trading pair symbol, e.g. "BTC-USDT".
    /// * `order_id` - Optional: Order ID.
    /// * `client_order_id` - Optional: Custom user order ID (1~40 chars, lowercase).
    ///
    /// # Returns
    /// * `ApiResponse<serde_json::Value>` - API response with order details.
    async fn get_swap_order_details(
        &self,
        symbol: &str,
        order_id: Option<i64>,
        client_order_id: Option<&str>,
    ) -> Result<ApiResponse<serde_json::Value>>;

    /// Query all currently open swap orders (open entrusts).
    ///
    /// Endpoint: GET /openApi/swap/v2/trade/openOrders
    ///
    /// [BingX API Documentation - Current All Open Orders](https://bingx-api.github.io/docs-v3/#/en/Swap/Trades%20Endpoints/Current%20All%20Open%20Orders)
    ///
    /// # Arguments
    /// * `symbol` - Optional: Symbol to filter open orders, e.g. "BTC-USDT".
    /// * `order_type` - Optional: Type of the order to filter.
    ///
    /// # Returns
    /// * `ApiResponse<serde_json::Value>` - API response with a list of open orders.
    async fn get_swap_open_orders(
        &self,
        symbol: Option<&str>,
        order_type: Option<SwapOrderType>,
    ) -> Result<ApiResponse<serde_json::Value>>;

    /// Cancel multiple swap orders in a batch (max 10 per request).
    ///
    /// Endpoint: DELETE /openApi/swap/v2/trade/batchOrders
    ///
    /// [BingX API Documentation - Cancel multiple orders](https://bingx-api.github.io/docs-v3/#/en/Swap/Trades%20Endpoints/Cancel%20multiple%20orders)
    ///
    /// # Arguments
    /// * `symbol` - Symbol string, e.g. "BTC-USDT".
    /// * `order_id_list` - Optional: List of up to 10 system order IDs to cancel.
    /// * `client_order_id_list` - Optional: List of up to 10 custom user order IDs to cancel.
    ///
    /// # Returns
    /// * `ApiResponse<serde_json::Value>` - API response indicating cancellation results.
    ///
    /// # Notes
    /// - At least one of order_id_list or client_order_id_list must be provided.
    /// - Signature required. UID rate limit: 5/sec.
    /// - Master and sub accounts supported.
    async fn cancel_swap_batch_orders(
        &self,
        symbol: &str,
        order_id_list: Option<&[i64]>,
        client_order_id_list: Option<&[&str]>,
    ) -> Result<ApiResponse<serde_json::Value>>;

    /// Query the position history of perpetual contracts for the specified symbol.
    ///
    /// Endpoint: GET /openApi/swap/v1/trade/positionHistory
    ///
    /// [BingX API Documentation - Query Position History](https://bingx-api.github.io/docs-v3/#/en/Swap/Trades%20Endpoints/Query%20Position%20History)
    ///
    /// # Arguments
    /// * `symbol` - Trading pair, e.g. "BTC-USDT".
    /// * `currency` - Optional: Quote currency, either USDC or USDT (as QuoteCurrency enum).
    /// * `position_id` - Optional: Position ID to filter by.
    /// * `start_ts` - Optional: Start timestamp in milliseconds (default: 90 days ago).
    /// * `end_ts` - Optional: End timestamp in milliseconds (default: now).
    /// * `page_index` - Optional: Page number (default: 1).
    /// * `page_size` - Optional: Page size, max 100 (default: 1000).
    ///
    /// # Returns
    /// * `ApiResponse<serde_json::Value>` - API response containing position history records.
    async fn get_swap_position_history(
        &self,
        symbol: &str,
        currency: Option<QuoteCurrency>,
        position_id: Option<i64>,
        start_ts: Option<i64>,
        end_ts: Option<i64>,
        page_index: Option<i32>,
        page_size: Option<i32>,
    ) -> Result<ApiResponse<serde_json::Value>>;

    /// Adjust the user's opening leverage in the specified symbol contract.
    ///
    /// Endpoint: POST /openApi/swap/v2/trade/leverage
    ///
    /// [BingX API Documentation - Set Leverage](https://bingx-api.github.io/docs-v3/#/en/Swap/Trades%20Endpoints/Set%20Leverage)
    ///
    /// # Arguments
    /// * `symbol` - Trading pair symbol (e.g., "BTC-USDT"), must include a hyphen.
    /// * `side` - Leverage side, e.g., `LeverageSide::Long`, `LeverageSide::Short`, or `LeverageSide::Both`.
    /// * `leverage` - Leverage value.
    ///
    /// # Returns
    /// * `ApiResponse<serde_json::Value>` - Response from BingX API.
    ///
    /// # Notes
    /// - UID rate limit: 5/sec.
    /// - Signature required.
    /// - Supported for master and sub accounts.
    async fn set_swap_leverage(
        &self,
        symbol: &str,
        side: PositionSide,
        leverage: i32,
    ) -> Result<ApiResponse<serde_json::Value>>;

    /// Set the position mode of perpetual contract (dual or single position mode).
    ///
    /// Endpoint: POST /openApi/swap/v1/positionSide/dual
    ///
    /// [BingX API Documentation - Set Position Mode](https://bingx-api.github.io/docs-v3/#/en/Swap/Trades%20Endpoints/Set%20Position%20Mode)
    ///
    /// # Arguments
    /// * `dual_side_position` - `bool`: `true` for dual position mode, `false` for single position mode.
    ///
    /// # Returns
    /// * `ApiResponse<serde_json::Value>` - Response from BingX API.
    ///
    /// # Notes
    /// - "dualSidePosition" POST param: `"true"` for dual, `"false"` for single.
    /// - UID rate limit: 4/sec per UID.
    /// - Signature required.
    /// - Supported for master and sub accounts.
    async fn set_swap_position_mode(
        &self,
        dual_side_position: bool,
    ) -> Result<ApiResponse<serde_json::Value>>;

    /// Get the position mode of perpetual contract (dual or single position mode).
    ///
    /// Endpoint: GET /openApi/swap/v1/positionSide/dual
    ///
    /// [BingX API Documentation - Query Position Mode](https://bingx-api.github.io/docs-v3/#/en/Swap/Trades%20Endpoints/Query%20position%20mode)
    ///
    /// # Returns
    /// * `ApiResponse<serde_json::Value>` - API response with the current position mode.
    ///
    /// # Notes
    /// - UID rate limit: 2/sec per UID.
    /// - Signature required.
    /// - Supported for master and sub accounts.
    async fn get_swap_position_mode(&self) -> Result<ApiResponse<serde_json::Value>>;

    /// Query leverage and available positions for the contract symbol.
    ///
    /// Endpoint: GET /openApi/swap/v2/trade/leverage
    ///
    /// [BingX API Documentation - Query Leverage and Available Positions](https://bingx-api.github.io/docs-v3/#/en/Swap/Trades%20Endpoints/Query%20Leverage%20and%20Available%20Positions)
    ///
    /// # Arguments
    /// * `symbol` - Trading pair symbol, e.g., "BTC-USDT".
    ///
    /// # Returns
    /// * `ApiResponse<serde_json::Value>` - API response with leverage and available positions.
    ///
    /// # Notes
    /// - UID rate limit: 5/sec per UID.
    /// - Signature required.
    /// - Supported for master and sub accounts.
    async fn get_swap_leverage_and_available_positions(
        &self,
        symbol: &str,
    ) -> Result<ApiResponse<serde_json::Value>>;

    /// Cancel all open swap orders for the account, or for provided symbol/type if specified.
    ///
    /// Endpoint: DELETE /openApi/swap/v2/trade/allOpenOrders
    ///
    /// [BingX API Documentation - Cancel All Open Orders](https://bingx-api.github.io/docs-v3/#/en/Swap/Trades%20Endpoints/Cancel%20All%20Open%20Orders)
    ///
    /// # Arguments
    /// * `symbol` - Optional trading pair symbol, e.g. "BTC-USDT". If None, cancels all orders for all symbols.
    /// * `order_type` - Optional order type to cancel (see `SwapOrderType`).
    ///
    /// # Returns
    /// * `ApiResponse<serde_json::Value>` - API response from BingX.
    ///
    /// # Notes
    /// - UID Rate Limit: 5/sec per UID.
    /// - Signature verification required.
    /// - Supported for master and sub accounts.
    async fn cancel_all_swap_open_orders(
        &self,
        symbol: Option<&str>,
        order_type: Option<SwapOrderType>,
    ) -> Result<ApiResponse<serde_json::Value>>;

    /// Change the user's margin mode on the specified symbol contract.
    ///
    /// Endpoint: POST /openApi/swap/v2/trade/marginType
    ///
    /// [BingX API Documentation - Change Margin Type](https://bingx-api.github.io/docs-v3/#/en/Swap/Trades%20Endpoints/Change%20Margin%20Type)
    ///
    /// # Arguments
    /// * `symbol` - Trading pair symbol, e.g., "BTC-USDT" (must contain '-').
    /// * `margin_type` - Margin mode as [`MarginMode`] enum ("ISOLATED", "CROSSED", or "SEPARATE_ISOLATED").
    ///
    /// # Returns
    /// * `ApiResponse<serde_json::Value>` - API response indicating margin type result.
    ///
    /// # Notes
    /// - UID Rate Limit: 2/second per UID.
    /// - Signature verification required.
    /// - Supported for master and sub accounts.
    async fn change_swap_margin_type(
        &self,
        symbol: &str,
        margin_type: MarginMode,
    ) -> Result<ApiResponse<serde_json::Value>>;

    /// Query the user's margin mode on the specified symbol contract.
    ///
    /// Endpoint: GET /openApi/swap/v2/trade/marginType
    ///
    /// [BingX API Documentation - Query Margin Type](https://bingx-api.github.io/docs-v3/#/en/Swap/Trades%20Endpoints/Query%20Margin%20Type)
    ///
    /// # Arguments
    /// * `symbol` - Trading pair symbol, e.g., "BTC-USDT" (must contain '-').
    ///
    /// # Returns
    /// * `ApiResponse<serde_json::Value>` - API response indicating the margin type for the contract.
    ///
    /// # Notes
    /// - UID Rate Limit: 2/second per UID.
    /// - Signature verification required.
    /// - Supported for master and sub accounts.
    async fn get_swap_margin_type(
        &self,
        symbol: &str,
    ) -> Result<ApiResponse<serde_json::Value>>;
}
