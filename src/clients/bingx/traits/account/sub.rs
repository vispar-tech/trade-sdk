use crate::{bingx::types::ApiResponse, error::Result};
use async_trait::async_trait;

#[async_trait]
pub trait SubAccountApi {
    /// Get API permission information.
    ///
    /// GET /openApi/v1/account/apiPermissions
    ///
    /// https://bingx-api.github.io/docs-v3/#/en/Account%20and%20Wallet/Sub-account%20Management/%20Query%20API%20KEY%20Permissions
    async fn get_api_permissions(&self) -> Result<ApiResponse<serde_json::Value>>;
}
