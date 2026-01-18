use crate::{
    bingx::types::{AccountType, ApiResponse},
    error::Result,
};
use async_trait::async_trait;
use serde_json::Value;

#[async_trait]
pub trait AccountApi {
    /// Get spot account balances from BingX API.
    ///
    /// See:
    /// https://bingx-api.github.io/docs-v3/#/en/Spot/Account%20Endpoints/Query%20Assets
    async fn get_spot_account_assets(&self) -> Result<ApiResponse<Value>>;

    /// Get asset overview for all or specific BingX account types.
    ///
    /// See: https://bingx-api.github.io/docs-v3/#/en/Spot/Account%20Endpoints/Asset%20overview
    ///
    /// # Arguments
    /// * `account_type` - (optional) AccountType enum. If omitted, returns all assets.
    async fn get_account_asset_overview(
        &self,
        account_type: Option<AccountType>,
    ) -> Result<ApiResponse<Value>>;
}
