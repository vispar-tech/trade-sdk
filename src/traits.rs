//! Traits for Bybit API client interfaces.

use async_trait::async_trait;
use std::collections::HashMap;

use crate::error::Result;
use crate::types::*;

/// Core HTTP client trait that all Bybit clients must implement
#[async_trait]
pub trait HttpClient {
    /// Send a GET request
    async fn get(
        &self,
        endpoint: &str,
        params: Option<&HashMap<String, String>>,
        auth: bool,
    ) -> Result<GenericResponse>;

    /// Send a POST request
    async fn post(
        &self,
        endpoint: &str,
        params: Option<&HashMap<String, String>>,
        auth: bool,
    ) -> Result<GenericResponse>;

    /// Send a PUT request
    async fn put(
        &self,
        endpoint: &str,
        params: Option<&HashMap<String, String>>,
        auth: bool,
    ) -> Result<GenericResponse>;

    /// Send a DELETE request
    async fn delete(
        &self,
        endpoint: &str,
        params: Option<&HashMap<String, String>>,
        auth: bool,
    ) -> Result<GenericResponse>;
}

/// Trait defining Bybit market data API endpoints.
///
/// This trait provides asynchronous methods to access various market data from the Bybit API.
/// Returned data mirrors the structure from Bybit's endpoints where possible.
#[async_trait]
pub trait MarketApi: HttpClient {
    /// Returns Bybit server time (in seconds and nanoseconds).
    async fn get_server_time(&self) -> Result<ApiResponse<serde_json::Value>>;

    /// Retrieves historical kline (candlestick) data.
    ///
    /// # Arguments
    /// * `symbol` – Symbol name, e.g. `"BTCUSDT"`.
    /// * `interval` – Kline interval (`1`, `3`, `5`, `15`, `30`, `60`, `120`, `240`, `360`, `720`, `"D"`, `"W"`, `"M"`).
    /// * `category` – Product type (`AllCategories`). Defaults to linear if not provided.
    ///   **Note:** `category` does **not** support `"option"`.
    /// * `start` – Start timestamp (milliseconds).
    /// * `end` – End timestamp (milliseconds).
    /// * `limit` – Number of records per page (1–1000; default: 200).
    ///
    /// # Returns
    /// Kline data including symbol, category, and a list of candles.  
    /// Each candle contains `[startTime, openPrice, highPrice, lowPrice, closePrice, volume, turnover]`.
    async fn get_kline(
        &self,
        symbol: &str,
        interval: &str,
        category: Option<&AllCategories>,
        start: Option<i64>,
        end: Option<i64>,
        limit: Option<i32>,
    ) -> Result<ApiResponse<serde_json::Value>>;

    /// Retrieves mark price kline data.
    async fn get_mark_price_kline(&self) -> Result<ApiResponse<serde_json::Value>>;

    /// Retrieves index price kline data.
    async fn get_index_price_kline(&self) -> Result<ApiResponse<serde_json::Value>>;

    /// Retrieves premium index price kline data.
    async fn get_premium_index_price_kline(&self) -> Result<ApiResponse<serde_json::Value>>;

    /// Retrieves instrument specification for active trading pairs.
    ///
    /// # Arguments
    /// * `category` – Product type: `"spot"`, `"linear"`, `"inverse"`, `"option"`.
    /// * `symbol` – Symbol name (e.g. `"BTCUSDT"`), uppercase.
    /// * `symbol_type` – Region/market classification for trading pair.
    /// * `status` – Filter by symbol status, e.g. `"Trading"`, `"PreLaunch"`, `"Delivering"`.
    /// * `base_coin` – Base coin (uppercase), for linear/inverse/option.
    /// * `limit` – Limit for data size per page (`1–1000`, default: 500).
    /// * `cursor` – Cursor for page pagination (from API response).
    ///
    /// # Returns
    /// Response struct containing the instruments info from Bybit.
    async fn get_instruments_info(
        &self,
        category: crate::types::AllCategories,
        symbol: Option<&str>,
        symbol_type: Option<&crate::types::SymbolType>,
        status: Option<&crate::types::InstrumentStatus>,
        base_coin: Option<&str>,
        limit: Option<i32>,
        cursor: Option<&str>,
    ) -> Result<ApiResponse<serde_json::Value>>;

    /// Returns the current orderbook.
    async fn get_orderbook(&self) -> Result<ApiResponse<serde_json::Value>>;

    /// Returns the current RPI orderbook.
    async fn get_rpi_orderbook(&self) -> Result<ApiResponse<serde_json::Value>>;

    /// Returns ticker information.
    async fn get_tickers(&self) -> Result<ApiResponse<serde_json::Value>>;

    /// Returns the funding rate history.
    async fn get_funding_rate_history(&self) -> Result<ApiResponse<serde_json::Value>>;

    /// Returns recent public trades.
    async fn get_recent_public_trades(&self) -> Result<ApiResponse<serde_json::Value>>;

    /// Returns open interest data.
    async fn get_open_interest(&self) -> Result<ApiResponse<serde_json::Value>>;

    /// Returns historical volatility information.
    async fn get_historical_volatility(&self) -> Result<ApiResponse<serde_json::Value>>;

    /// Returns insurance pool information.
    async fn get_insurance_pool(&self) -> Result<ApiResponse<serde_json::Value>>;

    /// Returns risk limit information.
    async fn get_risk_limit(&self) -> Result<ApiResponse<serde_json::Value>>;

    /// Returns delivery price information.
    async fn get_delivery_price(&self) -> Result<ApiResponse<serde_json::Value>>;

    /// Returns new delivery price information.
    async fn get_new_delivery_price(&self) -> Result<ApiResponse<serde_json::Value>>;

    /// Returns long-short ratio.
    async fn get_long_short_ratio(&self) -> Result<ApiResponse<serde_json::Value>>;

    /// Returns index price components.
    async fn get_index_price_components(&self) -> Result<ApiResponse<serde_json::Value>>;

    /// Returns order price limit info.
    async fn get_order_price_limit(&self) -> Result<ApiResponse<serde_json::Value>>;

    /// Returns ADL alert data.
    async fn get_adl_alert(&self) -> Result<ApiResponse<serde_json::Value>>;

    /// Returns the fee group structure for markets.
    async fn get_fee_group_structure(&self) -> Result<ApiResponse<serde_json::Value>>;
}

/// Trade management HTTP methods.
#[async_trait]
pub trait TradeApi: HttpClient {
    /// Places an order using the supplied parameters.
    ///
    /// Only fields defined in [`PlaceOrderParams`] are used.
    ///
    /// # Arguments
    /// * `category` - Product type ("linear", "inverse", "spot", "option").
    /// * `params` - Order parameters.
    ///
    /// # Returns
    /// Bybit order creation response (includes orderId, orderLinkId).
    async fn place_order(
        &self,
        category: crate::types::AllCategories,
        params: &crate::types::PlaceOrderParams,
    ) -> Result<ApiResponse<serde_json::Value>>;

    /// Amend an existing unfilled or partially filled order.
    ///
    /// Updates order parameters of an active order.
    async fn amend_order(&self) -> Result<ApiResponse<serde_json::Value>>;

    /// Cancels a single order.
    ///
    /// Cancels the order identified by `order_id` or a user-supplied `order_link_id`.
    ///
    /// # Arguments
    /// * `category` - Product type.
    /// * `symbol` - Symbol name (e.g. "BTCUSDT"), uppercase.
    /// * `order_id` - System order ID (optional).
    /// * `order_link_id` - User customized order ID (optional).
    /// * `order_filter` - For spot: Order, tpslOrder, StopOrder (optional).
    ///
    /// One of `order_id` or `order_link_id` is required.
    async fn cancel_order(
        &self,
        category: crate::types::AllCategories,
        symbol: &str,
        order_id: Option<&str>,
        order_link_id: Option<&str>,
        order_filter: Option<&crate::types::CancelOrderFilter>,
    ) -> Result<ApiResponse<serde_json::Value>>;

    /// Retrieves open and closed orders.
    ///
    /// Queries unfilled/partially filled orders (real-time), and up to 500 recently closed orders.
    ///
    /// # Arguments
    /// * `category` - Product type.
    /// * `symbol`, `base_coin`, `settle_coin`, `order_id`, `order_link_id` - Filters (optional).
    /// * `open_only` - Only open orders (optional).
    /// * `order_filter` - Filter by order type (optional).
    /// * `limit` - Maximum number of results (optional).
    /// * `cursor` - Pagination cursor (optional).
    async fn get_open_and_closed_orders(
        &self,
        category: crate::types::AllCategories,
        symbol: Option<&str>,
        base_coin: Option<&str>,
        settle_coin: Option<&str>,
        order_id: Option<&str>,
        order_link_id: Option<&str>,
        open_only: Option<bool>,
        order_filter: Option<&crate::types::OrderFilter>,
        limit: Option<i32>,
        cursor: Option<&str>,
    ) -> Result<ApiResponse<serde_json::Value>>;

    /// Cancels all active orders for the given category and optional filters.
    ///
    /// # Arguments
    /// * `category` - Product type.
    /// * `symbol`, `base_coin`, `settle_coin` - Use for filtering (at least one often required).
    /// * `order_filter`, `stop_order_type` - Further filter orders (optional).
    async fn cancel_all_orders(
        &self,
        category: crate::types::AllCategories,
        symbol: Option<&str>,
        base_coin: Option<&str>,
        settle_coin: Option<&str>,
        order_filter: Option<&crate::types::OrderFilter>,
        stop_order_type: Option<&str>,
    ) -> Result<ApiResponse<serde_json::Value>>;

    /// Retrieves order history.
    ///
    /// Queries order history, with filters optionally supplied in params.
    ///
    /// # Arguments
    /// * `category` - Product type.
    /// * `params` - Query parameters (optional).
    async fn get_order_history(
        &self,
        category: crate::types::AllCategories,
        params: Option<&crate::types::GetOrderHistoryParams>,
    ) -> Result<ApiResponse<serde_json::Value>>;

    /// Retrieves trade history (up to 2 years).
    async fn get_trade_history(&self) -> Result<ApiResponse<serde_json::Value>>;

    /// Batch submit multiple orders in a single request.
    ///
    /// # Arguments
    /// * `category` - Product type.
    /// * `orders` - List of order parameters.
    async fn batch_place_order(
        &self,
        category: crate::types::AllCategories,
        orders: &[PlaceOrderParams],
    ) -> Result<ApiResponse<serde_json::Value>>;

    /// Batch amend multiple orders in a single request.
    async fn batch_amend_order(&self) -> Result<ApiResponse<serde_json::Value>>;

    /// Batch cancel multiple orders in a single request.
    ///
    /// # Arguments
    /// * `category` - Product type.
    /// * `orders` - List of cancellation parameters.
    async fn batch_cancel_order(
        &self,
        category: crate::types::AllCategories,
        orders: &[CancelOrderParams],
    ) -> Result<ApiResponse<serde_json::Value>>;

    /// Gets borrow quota for spot trading.
    async fn get_borrow_quota_spot(&self) -> Result<ApiResponse<serde_json::Value>>;

    /// Sets Dynamic Contract Parameters (DCP).
    async fn set_dcp(&self) -> Result<ApiResponse<serde_json::Value>>;

    /// Pre-check order before placement.
    async fn pre_check_order(&self) -> Result<ApiResponse<serde_json::Value>>;
}

/// Trait defining Bybit account API endpoints.
///
/// This trait provides asynchronous methods to access and manage Bybit account information,
/// wallets, margin modes, and other account-related features.
///
/// Returned data generally mirrors the structure from Bybit's endpoints where possible.
#[async_trait]
pub trait AccountApi: HttpClient {
    /// Gets wallet balance from Bybit API.
    ///
    /// # Arguments
    /// * `account_type` - Account type ("UNIFIED", "CONTRACT", "SPOT", "FUND", "OPTION", "INVESTMENT"). Default is "UNIFIED".
    /// * `coin` - Optional. A single coin (e.g. "BTC") or comma-separated list of coins (e.g. "BTC,ETH,USDT"). If omitted, returns all coins.
    ///
    /// # Returns
    /// Bybit wallet balance response.
    async fn get_wallet_balance(
        &self,
        account_type: Option<crate::types::AccountType>,
        coin: Option<&str>,
    ) -> Result<ApiResponse<serde_json::Value>>;

    /// Gets transferable amount (Unified account).
    async fn get_transferable_amount(&self) -> Result<ApiResponse<serde_json::Value>>;

    /// Gets transaction log (Unified Trading Account).
    async fn get_transaction_log(&self) -> Result<ApiResponse<serde_json::Value>>;

    /// Gets account info from Bybit API.
    ///
    /// # Returns
    /// Bybit account info response.
    async fn get_account_info(&self) -> Result<ApiResponse<serde_json::Value>>;

    /// Gets account instruments info.
    async fn get_account_instruments_info(&self) -> Result<ApiResponse<serde_json::Value>>;

    /// Performs manual borrow.
    async fn manual_borrow(&self) -> Result<ApiResponse<serde_json::Value>>;

    /// Manually repays without asset conversion.
    async fn manual_repay_without_asset_conversion(&self)
        -> Result<ApiResponse<serde_json::Value>>;

    /// Manually repays liability.
    async fn manual_repay(&self) -> Result<ApiResponse<serde_json::Value>>;

    /// Gets fee rate.
    async fn get_fee_rate(&self) -> Result<ApiResponse<serde_json::Value>>;

    /// Gets collateral info.
    async fn get_collateral_info(&self) -> Result<ApiResponse<serde_json::Value>>;

    /// Gets DCP (Dynamic Collateral Portfolio) info.
    async fn get_dcp_info(&self) -> Result<ApiResponse<serde_json::Value>>;

    /// Sets collateral coin.
    async fn set_collateral_coin(&self) -> Result<ApiResponse<serde_json::Value>>;

    /// Sets margin mode from Bybit API.
    ///
    /// # Arguments
    /// * `set_margin_mode` - Margin mode to set (Isolated, Regular, Portfolio).
    ///
    /// # Returns
    /// Bybit response for setting margin mode.
    async fn set_margin_mode(
        &self,
        set_margin_mode: crate::types::MarginMode,
    ) -> Result<ApiResponse<serde_json::Value>>;

    /// Sets spot hedging.
    async fn set_spot_hedging(&self) -> Result<ApiResponse<serde_json::Value>>;

    /// Gets borrow history (up to 2 years).
    async fn get_borrow_history(&self) -> Result<ApiResponse<serde_json::Value>>;

    /// Batch sets collateral coin.
    async fn batch_set_collateral_coin(&self) -> Result<ApiResponse<serde_json::Value>>;

    /// Gets coin greeks.
    async fn get_coin_greeks(&self) -> Result<ApiResponse<serde_json::Value>>;

    /// Gets Market Maker Protection (MMP) state.
    async fn get_mmp_state(&self) -> Result<ApiResponse<serde_json::Value>>;

    /// Resets Market Maker Protection (MMP).
    async fn reset_mmp(&self) -> Result<ApiResponse<serde_json::Value>>;

    /// Sets Market Maker Protection (MMP).
    async fn set_mmp(&self) -> Result<ApiResponse<serde_json::Value>>;

    /// Gets SMP (Single Market Position) group ID.
    async fn get_smp_group_id(&self) -> Result<ApiResponse<serde_json::Value>>;

    /// Gets trade behaviour setting.
    async fn get_trade_behaviour_setting(&self) -> Result<ApiResponse<serde_json::Value>>;

    /// Sets limit price behaviour.
    async fn set_limit_price_behaviour(&self) -> Result<ApiResponse<serde_json::Value>>;

    /// Repays liability.
    async fn repay_liability(&self) -> Result<ApiResponse<serde_json::Value>>;

    /// Upgrades to unified account pro.
    async fn upgrade_to_unified_account_pro(&self) -> Result<ApiResponse<serde_json::Value>>;
}

/// Trait defining Bybit position management HTTP API endpoints.
///
/// This trait provides asynchronous methods to access and manage Bybit position-related endpoints,
/// such as position info, leverage, position mode, trading stop, and more.
///
/// Returned data generally mirrors the structure from Bybit's endpoints where possible.
#[async_trait]
pub trait PositionApi: HttpClient {
    /// Gets position info from Bybit API.
    ///
    /// # Arguments
    /// * `category` - Product type ("linear", "inverse", "option").  
    ///   **Note:** `category` does **not** support `"spot"`.
    /// * `symbol` - Optional symbol name, e.g., "BTCUSDT", uppercase.
    /// * `base_coin` - Optional. Base coin (option only).
    /// * `settle_coin` - Optional settle coin (linear: either symbol or settleCoin required).
    /// * `limit` - Optional. Limit for data size per page [1, 200]. Default: 20.
    /// * `cursor` - Optional. Cursor for pagination.
    ///
    /// # Returns
    /// Bybit position info response containing list of positions.
    async fn get_position_info(
        &self,
        category: crate::types::AllCategories,
        symbol: Option<&str>,
        base_coin: Option<&str>,
        settle_coin: Option<&str>,
        limit: Option<u32>,
        cursor: Option<&str>,
    ) -> Result<ApiResponse<serde_json::Value>>;

    /// Sets leverage from Bybit API.
    ///
    /// # Arguments
    /// * `category` - Product type ("linear", "inverse").
    ///   **Note:** `category` does **not** support `"spot"` and `"option"`.
    /// * `symbol` - Symbol name, e.g., "BTCUSDT", uppercase.
    /// * `buy_leverage` - Buy leverage [1, max leverage].
    /// * `sell_leverage` - Sell leverage [1, max leverage].
    ///
    /// # Returns
    /// Bybit set leverage response.
    async fn set_leverage(
        &self,
        category: crate::types::AllCategories,
        symbol: &str,
        buy_leverage: u32,
        sell_leverage: u32,
    ) -> Result<ApiResponse<serde_json::Value>>;

    /// Switches position mode from Bybit API.
    ///
    /// Supports switching between one-way mode (0) and hedge mode (3).
    ///
    /// # Arguments
    /// * `category` - Product type (e.g., "linear" for USDT contract).
    ///   **Note:** Only supports `"linear"` category.
    /// * `mode` - Position mode (0: one-way, 3: hedge).
    /// * `symbol` - Optional symbol name.
    /// * `coin` - Optional coin.
    ///
    /// # Returns
    /// Bybit switch position mode response.
    async fn switch_position_mode(
        &self,
        category: crate::types::AllCategories,
        mode: u8,
        symbol: Option<&str>,
        coin: Option<&str>,
    ) -> Result<ApiResponse<serde_json::Value>>;

    /// Sets trading stop (take profit, stop loss, trailing stop) for a position.
    ///
    /// # Arguments
    /// * `category` - Product type ("linear", "inverse").
    ///   **Note:** `category` does **not** support `"spot"` and `"option"`.
    /// * `params` - Trading stop parameters as a JSON object.
    ///
    /// Refer to Bybit API docs for full parameter list.
    ///
    /// # Returns
    /// Bybit trading stop response.
    async fn set_trading_stop(
        &self,
        category: crate::types::AllCategories,
        params: &crate::types::SetTradingStopParams,
    ) -> Result<ApiResponse<serde_json::Value>>;

    /// Sets auto add margin for a position.
    async fn set_auto_add_margin(&self) -> Result<ApiResponse<serde_json::Value>>;

    /// Adds or reduces margin for a position.
    async fn add_or_reduce_margin(&self) -> Result<ApiResponse<serde_json::Value>>;

    /// Gets closed PnL from Bybit API.
    ///
    /// # Arguments
    /// * `category` - Product type (e.g., "linear").
    ///   **Note:** Only supports `"linear"` category.
    /// * `symbol` - Optional symbol.
    /// * `start_time` - Optional start timestamp (ms).
    /// * `end_time` - Optional end timestamp (ms).
    /// * `limit` - Optional. Limit for data size per page [1, 100]. Default: 50.
    /// * `cursor` - Optional for pagination.
    async fn get_closed_pnl(
        &self,
        category: crate::types::AllCategories,
        symbol: Option<&str>,
        start_time: Option<u64>,
        end_time: Option<u64>,
        limit: Option<u32>,
        cursor: Option<&str>,
    ) -> Result<ApiResponse<serde_json::Value>>;

    /// Gets closed options positions (up to 6 months).
    async fn get_closed_options_positions(&self) -> Result<ApiResponse<serde_json::Value>>;

    /// Moves position.
    async fn move_position(&self) -> Result<ApiResponse<serde_json::Value>>;

    /// Gets move position history.
    async fn get_move_position_history(&self) -> Result<ApiResponse<serde_json::Value>>;

    /// Confirms new risk limit.
    async fn confirm_new_risk_limit(&self) -> Result<ApiResponse<serde_json::Value>>;
}
