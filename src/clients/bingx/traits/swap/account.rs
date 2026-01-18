use crate::{bingx::types::ApiResponse, error::Result};

use async_trait::async_trait;

/// BingX Swap Account related API methods.
///
/// This trait defines methods for managing swap trading accounts
/// and account-related operations, following the style of `CommonApi`.
#[async_trait]
pub trait AccountApi {
    /// Retrieve information on user's Perpetual Swap positions.
    ///
    /// GET /openApi/swap/v2/user/positions
    ///
    /// [BingX API Documentation - Query position data](https://bingx-api.github.io/docs-v3/#/en/Swap/Account%20Endpoints/Query%20position%20data)
    ///
    /// # Arguments
    /// * `symbol` - Optionally filter by symbol (e.g., "BTC-USDT"). If `None`, query all positions.
    ///
    /// # Returns
    /// Returns an `ApiResponse` containing position data.
    async fn get_swap_positions(
        &self,
        symbol: Option<&str>,
    ) -> Result<ApiResponse<serde_json::Value>>;

    /// Retrieve user's Perpetual Swap account balance.
    ///
    /// GET /openApi/swap/v3/user/balance
    ///
    /// [BingX API Documentation - Query account data](https://bingx-api.github.io/docs-v3/#/en/Swap/Account%20Endpoints/Query%20account%20data)
    ///
    /// # Returns
    /// Returns an `ApiResponse` containing account balance data.
    async fn get_swap_account_balance(&self) -> Result<ApiResponse<serde_json::Value>>;
}
