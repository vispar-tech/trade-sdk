use async_trait::async_trait;

use crate::bingx::traits::common::CommonApi;
use crate::bingx::types::ApiResponse;
use crate::bingx::BingxClient;
use crate::error::Result;
use crate::http::HttpClient;

use crate::bingx::BINGX_IMPLEMENTED;
use linkme::distributed_slice;

#[distributed_slice(BINGX_IMPLEMENTED)]
static GET_SERVER_TIME: &str = "get_server_time";

#[async_trait]
impl CommonApi for BingxClient {
    async fn get_server_time(&self) -> Result<ApiResponse<serde_json::Value>> {
        let response = self
            .get("/openApi/swap/v2/server/time", None, false)
            .await?;
        Ok(response.into_api_response())
    }
}
