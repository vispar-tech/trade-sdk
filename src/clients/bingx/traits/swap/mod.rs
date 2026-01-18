//! Swap trading API traits for BingX API client.

mod account;
mod market;
mod trade;

pub use account::AccountApi;
pub use market::MarketApi;
pub use trade::TradeApi;
