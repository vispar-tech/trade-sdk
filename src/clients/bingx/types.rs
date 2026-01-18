use serde::{Deserialize, Serialize};

/// Supported quote currencies: "USDT", "USDC"
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum QuoteCurrency {
    USDT,
    USDC,
}

impl std::fmt::Display for QuoteCurrency {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let s = match self {
            QuoteCurrency::USDT => "USDT",
            QuoteCurrency::USDC => "USDC",
        };
        write!(f, "{s}")
    }
}

/// Status of a spot order for BingX API.
/// "FILLED", "CANCELED", "FAILED"
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum SpotOrderStatus {
    Filled,
    Canceled,
    Failed,
}

impl std::fmt::Display for SpotOrderStatus {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let s = match self {
            SpotOrderStatus::Filled => "FILLED",
            SpotOrderStatus::Canceled => "CANCELED",
            SpotOrderStatus::Failed => "FAILED",
        };
        write!(f, "{s}")
    }
}

/// Spot order type for BingX API.
/// "MARKET", "LIMIT", "TAKE_STOP_LIMIT", "TAKE_STOP_MARKET", "TRIGGER_LIMIT", "TRIGGER_MARKET"
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SpotOrderType {
    Market,
    Limit,
    TakeStopLimit,
    TakeStopMarket,
    TriggerLimit,
    TriggerMarket,
}

impl std::fmt::Display for SpotOrderType {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let s = match self {
            SpotOrderType::Market => "MARKET",
            SpotOrderType::Limit => "LIMIT",
            SpotOrderType::TakeStopLimit => "TAKE_STOP_LIMIT",
            SpotOrderType::TakeStopMarket => "TAKE_STOP_MARKET",
            SpotOrderType::TriggerLimit => "TRIGGER_LIMIT",
            SpotOrderType::TriggerMarket => "TRIGGER_MARKET",
        };
        write!(f, "{s}")
    }
}

/// BingX account types.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccountType {
    #[serde(rename = "spot")]
    Spot,
    #[serde(rename = "stdFutures")]
    StdFutures,
    #[serde(rename = "coinMPerp")]
    CoinMPerp,
    #[serde(rename = "USDTMPerp")]
    UsdtMPerp,
    #[serde(rename = "copyTrading")]
    CopyTrading,
    #[serde(rename = "grid")]
    Grid,
    #[serde(rename = "eran")]
    Eran,
    #[serde(rename = "c2c")]
    C2C,
}

impl std::fmt::Display for AccountType {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let s = match self {
            AccountType::Spot => "spot",
            AccountType::StdFutures => "stdFutures",
            AccountType::CoinMPerp => "coinMPerp",
            AccountType::UsdtMPerp => "USDTMPerp",
            AccountType::CopyTrading => "copyTrading",
            AccountType::Grid => "grid",
            AccountType::Eran => "eran",
            AccountType::C2C => "c2c",
        };
        write!(f, "{s}")
    }
}

/// Margin mode for BingX swap accounts.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MarginMode {
    Isolated,
    Crossed,
    SeparateIsolated,
}

impl std::fmt::Display for MarginMode {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let s = match self {
            MarginMode::Isolated => "ISOLATED",
            MarginMode::Crossed => "CROSSED",
            MarginMode::SeparateIsolated => "SEPARATE_ISOLATED",
        };
        write!(f, "{s}")
    }
}

/// Swap order types for BingX swap API.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SwapOrderType {
    Limit,
    #[default]
    Market,
    StopMarket,
    TakeProfitMarket,
    Stop,
    TakeProfit,
    TriggerLimit,
    TriggerMarket,
    TrailingStopMarket,
    TrailingTpSl,
}

impl std::fmt::Display for SwapOrderType {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let s = match self {
            SwapOrderType::Limit => "LIMIT",
            SwapOrderType::Market => "MARKET",
            SwapOrderType::StopMarket => "STOP_MARKET",
            SwapOrderType::TakeProfitMarket => "TAKE_PROFIT_MARKET",
            SwapOrderType::Stop => "STOP",
            SwapOrderType::TakeProfit => "TAKE_PROFIT",
            SwapOrderType::TriggerLimit => "TRIGGER_LIMIT",
            SwapOrderType::TriggerMarket => "TRIGGER_MARKET",
            SwapOrderType::TrailingStopMarket => "TRAILING_STOP_MARKET",
            SwapOrderType::TrailingTpSl => "TRAILING_TP_SL",
        };
        write!(f, "{s}")
    }
}

/// Take-profit / Stop-loss order type for BingX swap API.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TpSlOrderType {
    Stop,
    TakeProfit,
    StopMarket,
    TakeProfitMarket,
}

/// Order side ("BUY" or "SELL") for BingX swap API.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "UPPERCASE")]
pub enum OrderSide {
    Sell,
    #[default]
    Buy,
}

/// Position side for BingX swap accounts ("BOTH", "LONG", "SHORT").
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum PositionSide {
    Both,
    Long,
    Short,
}

impl std::fmt::Display for PositionSide {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let s = match self {
            PositionSide::Both => "BOTH",
            PositionSide::Long => "LONG",
            PositionSide::Short => "SHORT",
        };
        write!(f, "{s}")
    }
}

/// Trigger price type for BingX swap API ("MARK_PRICE", "CONTRACT_PRICE", "INDEX_PRICE").
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TriggerPriceType {
    MarkPrice,
    ContractPrice,
    IndexPrice,
}

/// Order execution time-in-force options for BingX swap API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TimeInForce {
    PostOnly,
    GTC,
    IOC,
    FOK,
}

/// Stop Guaranteed option for BingX swap TP/SL orders.
/// Allowed: "true", "false", "cutfee"
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StopGuaranteed {
    True,
    False,
    CutFee,
}

/// Structured type for BingX swap takeProfit/stopLoss fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TpSlStruct {
    /// Order type for TP/SL ("STOP", "TAKE_PROFIT", "STOP_MARKET", "TAKE_PROFIT_MARKET")
    #[serde(rename = "type")]
    pub order_type: SwapOrderType,
    /// Stop price for TP/SL
    pub stop_price: f64,
    /// Order price for TP/SL
    pub price: f64,
    /// Trigger price type ("MARK_PRICE", "CONTRACT_PRICE", "INDEX_PRICE")
    pub working_type: TriggerPriceType,
}

/// Request parameters for creating/modifying an order on BingX.
///
/// There must be a hyphen "-" in the trading pair symbol (e.g. BTC-USDT).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlaceSwapOrderParams {
    /// Symbol, trading pair (e.g. BTC-USDT)
    pub symbol: String,

    /// Order type (e.g. LIMIT, MARKET, STOP_MARKET, etc.)
    #[serde(rename = "type")]
    pub order_type: SwapOrderType,

    /// Side ("BUY" or "SELL")
    pub side: OrderSide,

    /// Position direction ("BOTH", "LONG", "SHORT"). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_side: Option<PositionSide>,

    /// Only for single position mode. Send as string "true"/"false" if needed.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::utils::as_str_bool"
    )]
    pub reduce_only: Option<bool>,

    /// Price, or trailing stop distance for certain order types. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,

    /// Order quantity in COIN. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<f64>,

    /// Quote order quantity, e.g. 100USDT. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_order_qty: Option<f64>,

    /// Trigger price for some order types. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<f64>,

    /// For trailing orders. Maximum: 1. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_rate: Option<f64>,

    /// Stop loss setting, only for STOP_MARKET/STOP. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_type: Option<String>,

    /// Take-profit order. Accepts TpSlStruct or stringified JSON. Optional.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::utils::serialize_as_json_string"
    )]
    pub take_profit: Option<TpSlStruct>,

    /// Stop-loss order. Accepts TpSlStruct or stringified JSON. Optional.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::utils::serialize_as_json_string"
    )]
    pub stop_loss: Option<TpSlStruct>,

    /// User-custom order ID (1-40 chars, lowercased). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,

    /// Order execution time-in-force. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TimeInForce>,

    /// Close all position after trigger. Optional, needs to be string "true"/"false".
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::utils::as_str_bool"
    )]
    pub close_position: Option<bool>,

    /// Used with trailing stop orders. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activation_price: Option<f64>,

    /// Guaranteed SL/TP feature. "true", "false", or "cutfee". Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_guaranteed: Option<StopGuaranteed>,

    /// Required when closing in Separate Isolated mode. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_id: Option<i64>,
}

/// BingX API response for deserialization (fields are received from API, not for sending)
#[derive(Debug, Clone, Deserialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub msg: String,
    #[serde(default)]
    pub data: T,
    #[serde(default, rename = "debugMsg")]
    pub debug_msg: Option<String>,
    #[serde(default, deserialize_with = "crate::utils::retryable_from_int")]
    pub retryable: Option<bool>,
}

/// Generic API response for deserialization (fields are received from API, not for sending)
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenericResponse {
    pub code: i32,
    pub msg: String,
    #[serde(default)]
    pub data: serde_json::Value,
    #[serde(default, rename = "debugMsg")]
    pub debug_msg: Option<String>,
    #[serde(default, deserialize_with = "crate::utils::retryable_from_int")]
    pub retryable: Option<bool>,
}

impl GenericResponse {
    /// Convert GenericResponse to ApiResponse<serde_json::Value>
    pub fn into_api_response(self) -> ApiResponse<serde_json::Value> {
        ApiResponse {
            code: self.code,
            msg: self.msg,
            data: self.data,
            debug_msg: self.debug_msg,
            retryable: self.retryable,
        }
    }
}

impl From<GenericResponse> for ApiResponse<serde_json::Value> {
    fn from(response: GenericResponse) -> Self {
        response.into_api_response()
    }
}
