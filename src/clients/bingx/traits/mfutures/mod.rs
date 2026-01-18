//! Futures-related mixins for BingX API client.

mod market;
mod trade;

pub use market::MarketApi;
pub use trade::TradeApi;
