use crate::bingx::traits::account::SubAccountApi;
use crate::bingx::types::ApiResponse;
use crate::bingx::BingxClient;
use crate::error::Result;
use crate::http::HttpClient;
use async_trait::async_trait;
use linkme::distributed_slice;
use serde_json::Value;

use crate::bingx::BINGX_IMPLEMENTED;

#[distributed_slice(BINGX_IMPLEMENTED)]
pub static GET_API_PERMISSIONS: &'static str = "get_api_permissions";

#[async_trait]
impl SubAccountApi for BingxClient {
    async fn get_api_permissions(&self) -> Result<ApiResponse<Value>> {
        let response = self
            .get("/openApi/v1/account/apiPermissions", None, true)
            .await?;
        Ok(response.into_api_response())
    }
}
