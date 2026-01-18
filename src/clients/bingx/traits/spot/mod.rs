//! Spot trading mixins for BingX API client.

mod account;
mod market;
mod trade;
mod wallet;

pub use account::AccountApi;
pub use market::MarketApi;
pub use trade::TradeApi;
pub use wallet::WalletApi;
