//! Type definitions for trade-sdk.
use crate::utils::{as_str_f64, as_str_opt};
use serde::{Deserialize, Serialize};

/// Enum for all possible instrument categories.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AllCategories {
    Spot,
    Linear,
    Inverse,
    Option,
}

impl std::fmt::Display for AllCategories {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let s = match self {
            AllCategories::Spot => "spot",
            AllCategories::Linear => "linear",
            AllCategories::Inverse => "inverse",
            AllCategories::Option => "option",
        };
        write!(f, "{s}")
    }
}

// Account types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AccountType {
    Unified,
    Fund,
}

impl std::fmt::Display for AccountType {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let s = match self {
            AccountType::Unified => "UNIFIED",
            AccountType::Fund => "FUND",
        };
        write!(f, "{s}")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MarginMode {
    IsolatedMargin,
    RegularMargin,
    PortfolioMargin,
}

impl std::fmt::Display for MarginMode {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let s = match self {
            MarginMode::IsolatedMargin => "ISOLATED_MARGIN",
            MarginMode::RegularMargin => "REGULAR_MARGIN",
            MarginMode::PortfolioMargin => "PORTFOLIO_MARGIN",
        };
        write!(f, "{s}")
    }
}

// Market types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InstrumentStatus {
    Trading,
    PreLaunch,
    Delivering,
}

impl std::fmt::Display for InstrumentStatus {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let s = match self {
            InstrumentStatus::Trading => "Trading",
            InstrumentStatus::PreLaunch => "PreLaunch",
            InstrumentStatus::Delivering => "Delivering",
        };
        write!(f, "{s}")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SymbolType {
    Innovation,
    Adventure,
    XStocks,
}

impl std::fmt::Display for SymbolType {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let s = match self {
            SymbolType::Innovation => "innovation",
            SymbolType::Adventure => "adventure",
            SymbolType::XStocks => "xstocks",
        };
        write!(f, "{s}")
    }
}

// Trade types
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum Side {
    #[default]
    Buy,
    Sell,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum PlaceOrderType {
    #[default]
    Market,
    Limit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MarketUnit {
    BaseCoin,
    QuoteCoin,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderPriceTriggerBy {
    LastPrice,
    IndexPrice,
    MarkPrice,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum TimeInForce {
    Gtc,
    Ioc,
    Fok,
}

/// Position index.
///
/// 0 = one-way mode position
/// 1 = Buy side of hedge-mode position
/// 2 = Sell side of hedge-mode position
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PositionIdx {
    /// one-way mode position
    #[serde(rename = "0")]
    OneWay = 0,

    /// Buy side of hedge-mode position
    #[serde(rename = "1")]
    HedgeBuy = 1,

    /// Sell side of hedge-mode position
    #[serde(rename = "2")]
    HedgeSell = 2,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TpSlTriggerBy {
    LastPrice,
    IndexPrice,
    MarkPrice,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TpSlMode {
    Full,
    Partial,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TpSlOrderType {
    Market,
    Limit,
}

/// Parameters for setting trading stop (TP/SL).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetTradingStopParams {
    // Required fields
    pub symbol: String,
    pub tpsl_mode: TpSlMode,       // Full or Partial
    pub position_idx: PositionIdx, // 0, 1, 2

    // Optional TP/SL fields (serialized as strings, skip if None)
    #[serde(skip_serializing_if = "Option::is_none", serialize_with = "as_str_opt")]
    pub take_profit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", serialize_with = "as_str_opt")]
    pub stop_loss: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", serialize_with = "as_str_opt")]
    pub trailing_stop: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_trigger_by: Option<TpSlTriggerBy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_trigger_by: Option<TpSlTriggerBy>,
    #[serde(skip_serializing_if = "Option::is_none", serialize_with = "as_str_opt")]
    pub active_price: Option<f64>,

    // Partial mode fields (serialized as strings, skip if None)
    #[serde(skip_serializing_if = "Option::is_none", serialize_with = "as_str_opt")]
    pub tp_size: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", serialize_with = "as_str_opt")]
    pub sl_size: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", serialize_with = "as_str_opt")]
    pub tp_limit_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", serialize_with = "as_str_opt")]
    pub sl_limit_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_order_type: Option<TpSlOrderType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_order_type: Option<TpSlOrderType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderFilter {
    #[serde(rename = "Order")]
    Order,
    #[serde(rename = "StopOrder")]
    StopOrder,
    #[serde(rename = "tpslOrder")]
    TpslOrder,
    #[serde(rename = "OcoOrder")]
    OcoOrder,
    #[serde(rename = "BidirectionalTpslOrder")]
    BidirectionalTpslOrder,
}

impl std::fmt::Display for OrderFilter {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let s = match self {
            OrderFilter::Order => "Order",
            OrderFilter::StopOrder => "StopOrder",
            OrderFilter::TpslOrder => "tpslOrder",
            OrderFilter::OcoOrder => "OcoOrder",
            OrderFilter::BidirectionalTpslOrder => "BidirectionalTpslOrder",
        };
        write!(f, "{s}")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CancelOrderFilter {
    #[serde(rename = "Order")]
    Order,
    #[serde(rename = "tpslOrder")]
    TpslOrder,
    #[serde(rename = "StopOrder")]
    StopOrder,
}

impl std::fmt::Display for CancelOrderFilter {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let s = match self {
            CancelOrderFilter::Order => "Order",
            CancelOrderFilter::TpslOrder => "tpslOrder",
            CancelOrderFilter::StopOrder => "StopOrder",
        };
        write!(f, "{s}")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderStatus {
    // Open status
    New,
    PartiallyFilled,
    Untriggered,
    // Closed status
    Rejected,
    PartiallyFilledCanceled,
    Filled,
    Cancelled,
    Triggered,
    Deactivated,
}

/// Parameters for querying order history.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderHistoryParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settle_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_filter: Option<OrderFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_status: Option<OrderStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

/// Parameters for canceling an order.
///
/// When serializing, either `order_id` or `order_link_id` must be provided.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderParams {
    pub symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
}

impl CancelOrderParams {
    /// Creates a new CancelOrderParams.
    ///
    /// # Panics
    ///
    /// Panics if both `order_id` and `order_link_id` are `None`.
    pub fn new<S: Into<String>>(
        symbol: S,
        order_id: Option<String>,
        order_link_id: Option<String>,
    ) -> Self {
        if order_id.is_none() && order_link_id.is_none() {
            panic!("Either order_id or order_link_id must be provided");
        }
        Self {
            symbol: symbol.into(),
            order_id,
            order_link_id,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrderParams {
    /// Parameters for placing an order.

    /// Symbol name, like BTCUSDT, uppercase only. (required)
    pub symbol: String,

    /// Whether to borrow.
    /// 0(default): false, spot trading
    /// 1: true, margin trading, make sure you turn on margin trading, and set the relevant currency as collateral
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_leverage: Option<i32>,

    /// Order side (required)
    pub side: Side,

    /// Order type (required)
    pub order_type: PlaceOrderType,

    /// Order quantity (required).
    /// For Spot: Market Buy order defaults to "by value". You can set `market_unit` to choose ordering by value or by quantity for market orders.
    /// For Perps, Futures & Options: Always order by quantity.
    /// For Perps & Futures: If qty="0" and you set `reduce_only=True` and `close_on_trigger=True`, you can close the position up to maxMktOrderQty or maxOrderQty (see "Get Instruments Info" for the relevant symbol).
    #[serde(serialize_with = "as_str_f64")]
    pub qty: f64, // needs to be str in serialization

    /// Select the unit for qty when creating Spot market orders. Optional.
    /// "baseCoin": For example, buy BTCUSDT, then "qty" unit is BTC.
    /// "quoteCoin": For example, sell BTCUSDT, then "qty" unit is USDT.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_unit: Option<MarketUnit>,

    /// Order price.
    /// Market orders will ignore this field.
    /// Please check the min price and price precision from the instrument info endpoint.
    /// If you have a position, price must be better than the liquidation price.
    #[serde(skip_serializing_if = "Option::is_none", serialize_with = "as_str_opt")]
    pub price: Option<f64>,

    /// The conditional order trigger price.
    /// For Perps & Futures: Set trigger_price > market price if you expect the price to rise to trigger your order. Otherwise, set trigger_price < market price.
    /// For Spot: Used for TP/SL and Conditional order trigger price.
    #[serde(skip_serializing_if = "Option::is_none", serialize_with = "as_str_opt")]
    pub trigger_price: Option<f64>,

    /// Trigger price type, Conditional order param for Perps & Futures. Valid for linear & inverse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_by: Option<OrderPriceTriggerBy>,

    /// Time in force for the order. Market orders will always use IOC. If not passed, GTC is used by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TimeInForce>,

    /// Position index. Used to identify positions in different position modes.
    /// Under hedge-mode, this param is required.
    /// 0: one-way mode; 1: hedge-mode Buy side; 2: hedge-mode Sell side
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_idx: Option<PositionIdx>,

    /// User customised order ID.
    /// A max of 36 characters. Combinations of numbers, letters (upper and lower cases), dashes, and underscores are supported.
    /// Futures & Perps: order_link_id is optional and must always be unique if provided.
    /// Options: order_link_id is required and must always be unique.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,

    /// Take profit price.
    /// Spot Limit order supports take profit, stop loss or limit take profit, limit stop loss when creating an order.
    #[serde(skip_serializing_if = "Option::is_none", serialize_with = "as_str_opt")]
    pub take_profit: Option<f64>,

    /// Stop loss price.
    /// Spot Limit order supports take profit, stop loss or limit take profit, limit stop loss when creating an order.
    #[serde(skip_serializing_if = "Option::is_none", serialize_with = "as_str_opt")]
    pub stop_loss: Option<f64>,

    /// The price type to trigger take profit. MarkPrice, IndexPrice, default: LastPrice. Valid for linear & inverse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_trigger_by: Option<TpSlTriggerBy>,

    /// The price type to trigger stop loss. MarkPrice, IndexPrice, default: LastPrice. Valid for linear & inverse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_trigger_by: Option<TpSlTriggerBy>,

    /// reduce_only specifies if the order is reduce-only.
    /// true means your position can only reduce in size if this order is triggered.
    /// You must specify it as true when you are about to close/reduce the position.
    /// When reduce_only is true, take profit/stop loss cannot be set.
    /// Valid for linear, inverse & option.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,

    /// TP/SL mode.
    /// Full: entire position for TP/SL. Then, tp_order_type or sl_order_type must be Market.
    /// Partial: partial position TP/SL (creates TP/SL orders with the qty you actually fill).
    ///   Limit TP/SL orders are only supported in Partial mode.
    ///   When creating limit TP/SL, tpsl_mode is required and must be Partial.
    /// Valid for linear & inverse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tpsl_mode: Option<TpSlMode>,

    /// The limit order price when take profit price is triggered.
    /// linear & inverse: only works when tpsl_mode=Partial and tp_order_type=Limit.
    /// Spot: required when the order has take_profit and tp_order_type=Limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_limit_price: Option<String>,

    /// The limit order price when stop loss price is triggered.
    /// linear & inverse: only works when tpsl_mode=Partial and sl_order_type=Limit.
    /// Spot: required when the order has stop_loss and sl_order_type=Limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_limit_price: Option<String>,

    /// The order type when take profit is triggered.
    /// linear & inverse: Market (default), Limit.
    /// For tpsl_mode=Full, only supports tp_order_type=Market.
    /// Spot: Market when you set take_profit,
    /// Limit when you set both take_profit and tp_limit_price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_order_type: Option<TpSlOrderType>,

    /// The order type when stop loss is triggered.
    /// linear & inverse: Market (default), Limit.
    ///   For tpsl_mode=Full, only supports sl_order_type=Market.
    /// Spot: Market when you set stop_loss,
    /// Limit when you set both stop_loss and sl_limit_price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_order_type: Option<TpSlOrderType>,
}

/// API response wrapper
/// Generic add for future support
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse<T> {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: T,
    #[serde(default)]
    pub ret_ext_info: serde_json::Value,
    pub time: u64,
}

/// Generic API response for deserialization
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenericResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    #[serde(default)]
    pub result: serde_json::Value,
    #[serde(default)]
    pub ret_ext_info: serde_json::Value,
    pub time: u64,
}

impl GenericResponse {
    /// Convert GenericResponse to ApiResponse<serde_json::Value>
    pub fn into_api_response(self) -> ApiResponse<serde_json::Value> {
        ApiResponse {
            ret_code: self.ret_code,
            ret_msg: self.ret_msg,
            result: self.result,
            ret_ext_info: self.ret_ext_info,
            time: self.time,
        }
    }
}

impl From<GenericResponse> for ApiResponse<serde_json::Value> {
    fn from(response: GenericResponse) -> Self {
        response.into_api_response()
    }
}
