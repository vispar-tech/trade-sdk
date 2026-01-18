//! Account API implementation.

use std::collections::HashMap;

use async_trait::async_trait;
use linkme::distributed_slice;
use serde_json::Value;

use crate::bybit::traits::AccountApi;
use crate::bybit::types::{AccountType, ApiResponse, MarginMode};
use crate::bybit::BybitClient;
use crate::bybit::BYBIT_IMPLEMENTED;
use crate::error::Result;
use crate::http::HttpClient;

#[distributed_slice(BYBIT_IMPLEMENTED)]
pub static GET_WALLET_BALANCE: &'static str = "get_wallet_balance";

#[distributed_slice(BYBIT_IMPLEMENTED)]
pub static GET_ACCOUNT_INFO: &'static str = "get_account_info";

#[distributed_slice(BYBIT_IMPLEMENTED)]
pub static SET_MARGIN_MODE: &'static str = "set_margin_mode";

#[async_trait]
impl AccountApi for BybitClient {
    async fn get_wallet_balance(
        &self,
        account_type: Option<AccountType>,
        coin: Option<&str>,
    ) -> Result<ApiResponse<Value>> {
        let mut params: HashMap<String, Value> = HashMap::new();

        params.insert(
            "accountType".to_string(),
            Value::String(account_type.unwrap_or(AccountType::Unified).to_string()),
        );
        if let Some(coin) = coin {
            params.insert("coin".to_string(), Value::String(coin.to_string()));
        }

        let response = self
            .get("/v5/account/wallet-balance", Some(&params), true)
            .await?;
        Ok(response.into_api_response())
    }

    async fn get_account_info(&self) -> Result<ApiResponse<Value>> {
        let response = self.get("/v5/account/info", None, true).await?;
        Ok(response.into_api_response())
    }

    async fn set_margin_mode(
        &self,
        set_margin_mode: MarginMode,
    ) -> Result<ApiResponse<Value>> {
        let mut params: HashMap<String, Value> = HashMap::new();
        params.insert(
            "setMarginMode".to_string(),
            Value::String(set_margin_mode.to_string()),
        );

        let response = self
            .post("/v5/account/set-margin-mode", Some(&params), true)
            .await?;
        Ok(response.into_api_response())
    }

    async fn get_transferable_amount(&self) -> Result<ApiResponse<Value>> {
        todo!("get_transferable_amount not implemented")
    }

    async fn get_transaction_log(&self) -> Result<ApiResponse<Value>> {
        todo!("get_transaction_log not implemented")
    }

    async fn get_account_instruments_info(&self) -> Result<ApiResponse<Value>> {
        todo!("get_account_instruments_info not implemented")
    }

    async fn manual_borrow(&self) -> Result<ApiResponse<Value>> {
        todo!("manual_borrow not implemented")
    }

    async fn manual_repay_without_asset_conversion(&self) -> Result<ApiResponse<Value>> {
        todo!("manual_repay_without_asset_conversion not implemented")
    }

    async fn manual_repay(&self) -> Result<ApiResponse<Value>> {
        todo!("manual_repay not implemented")
    }

    async fn get_fee_rate(&self) -> Result<ApiResponse<Value>> {
        todo!("get_fee_rate not implemented")
    }

    async fn get_collateral_info(&self) -> Result<ApiResponse<Value>> {
        todo!("get_collateral_info not implemented")
    }

    async fn get_dcp_info(&self) -> Result<ApiResponse<Value>> {
        todo!("get_dcp_info not implemented")
    }

    async fn set_collateral_coin(&self) -> Result<ApiResponse<Value>> {
        todo!("set_collateral_coin not implemented")
    }

    async fn set_spot_hedging(&self) -> Result<ApiResponse<Value>> {
        todo!("set_spot_hedging not implemented")
    }

    async fn get_borrow_history(&self) -> Result<ApiResponse<Value>> {
        todo!("get_borrow_history not implemented")
    }

    async fn batch_set_collateral_coin(&self) -> Result<ApiResponse<Value>> {
        todo!("batch_set_collateral_coin not implemented")
    }

    async fn get_coin_greeks(&self) -> Result<ApiResponse<Value>> {
        todo!("get_coin_greeks not implemented")
    }

    async fn get_mmp_state(&self) -> Result<ApiResponse<Value>> {
        todo!("get_mmp_state not implemented")
    }

    async fn reset_mmp(&self) -> Result<ApiResponse<Value>> {
        todo!("reset_mmp not implemented")
    }

    async fn set_mmp(&self) -> Result<ApiResponse<Value>> {
        todo!("set_mmp not implemented")
    }

    async fn get_smp_group_id(&self) -> Result<ApiResponse<Value>> {
        todo!("get_smp_group_id not implemented")
    }

    async fn get_trade_behaviour_setting(&self) -> Result<ApiResponse<Value>> {
        todo!("get_trade_behaviour_setting not implemented")
    }

    async fn set_limit_price_behaviour(&self) -> Result<ApiResponse<Value>> {
        todo!("set_limit_price_behaviour not implemented")
    }

    async fn repay_liability(&self) -> Result<ApiResponse<Value>> {
        todo!("repay_liability not implemented")
    }

    async fn upgrade_to_unified_account_pro(&self) -> Result<ApiResponse<Value>> {
        todo!("upgrade_to_unified_account_pro not implemented")
    }
}
