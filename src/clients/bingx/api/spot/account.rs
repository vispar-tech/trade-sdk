use crate::{
    bingx::{
        traits::spot::AccountApi,
        types::{AccountType, ApiResponse},
        BingxClient,
    },
    error::Result,
    http::HttpClient,
};
use async_trait::async_trait;
use serde_json::Value;
use linkme::distributed_slice;
use crate::bingx::BINGX_IMPLEMENTED;

#[distributed_slice(BINGX_IMPLEMENTED)]
static GET_SPOT_ACCOUNT_ASSETS: &str = "get_spot_account_assets";

#[distributed_slice(BINGX_IMPLEMENTED)]
static GET_ACCOUNT_ASSET_OVERVIEW: &str = "get_account_asset_overview";



#[async_trait]
impl AccountApi for BingxClient {
    async fn get_spot_account_assets(&self) -> Result<ApiResponse<Value>> {
        let response = self
            .get("/openApi/spot/v1/account/balance", None, true)
            .await?;
        Ok(response.into_api_response())
    }

    async fn get_account_asset_overview(
        &self,
        account_type: Option<AccountType>,
    ) -> Result<ApiResponse<Value>> {
        let mut params: std::collections::HashMap<String, Value> = std::collections::HashMap::new();
        if let Some(account_type) = account_type {
            params.insert(
                "accountType".to_string(),
                Value::String(account_type.to_string()),
            );
        }
        let response = self
            .get("/openApi/account/v1/allAccountBalance", Some(&params), true)
            .await?;
        Ok(response.into_api_response())
    }
}