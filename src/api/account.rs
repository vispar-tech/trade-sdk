//! Account API implementation.

use std::collections::HashMap;

use async_trait::async_trait;

use crate::{
    error::Result,
    traits::{AccountApi, HttpClient},
    types::{AccountType, ApiResponse, MarginMode},
};

#[async_trait]
impl<T: HttpClient + Send + Sync> AccountApi for T {
    async fn get_wallet_balance(
        &self,
        account_type: Option<AccountType>,
        coin: Option<&str>,
    ) -> Result<ApiResponse<serde_json::Value>> {
        let mut params = HashMap::new();

        params.insert(
            "accountType".to_string(),
            account_type.unwrap_or(AccountType::Unified).to_string(),
        );
        if let Some(coin) = coin {
            params.insert("coin".to_string(), coin.to_string());
        }

        let response = self
            .get("/v5/account/wallet-balance", Some(&params), true)
            .await?;
        Ok(response.into_api_response())
    }

    async fn get_account_info(&self) -> Result<ApiResponse<serde_json::Value>> {
        let response = self.get("/v5/account/info", None, true).await?;
        Ok(response.into_api_response())
    }

    async fn set_margin_mode(
        &self,
        set_margin_mode: MarginMode,
    ) -> Result<ApiResponse<serde_json::Value>> {
        let mut params = HashMap::new();
        params.insert("setMarginMode".to_string(), set_margin_mode.to_string());

        let response = self
            .post("/v5/account/set-margin-mode", Some(&params), true)
            .await?;
        Ok(response.into_api_response())
    }

    async fn get_transferable_amount(&self) -> Result<ApiResponse<serde_json::Value>> {
        todo!("get_transferable_amount not implemented")
    }

    async fn get_transaction_log(&self) -> Result<ApiResponse<serde_json::Value>> {
        todo!("get_transaction_log not implemented")
    }

    async fn get_account_instruments_info(&self) -> Result<ApiResponse<serde_json::Value>> {
        todo!("get_account_instruments_info not implemented")
    }

    async fn manual_borrow(&self) -> Result<ApiResponse<serde_json::Value>> {
        todo!("manual_borrow not implemented")
    }

    async fn manual_repay_without_asset_conversion(
        &self
    ) -> Result<ApiResponse<serde_json::Value>> {
        todo!("manual_repay_without_asset_conversion not implemented")
    }

    async fn manual_repay(&self) -> Result<ApiResponse<serde_json::Value>> {
        todo!("manual_repay not implemented")
    }

    async fn get_fee_rate(&self) -> Result<ApiResponse<serde_json::Value>> {
        todo!("get_fee_rate not implemented")
    }

    async fn get_collateral_info(&self) -> Result<ApiResponse<serde_json::Value>> {
        todo!("get_collateral_info not implemented")
    }

    async fn get_dcp_info(&self) -> Result<ApiResponse<serde_json::Value>> {
        todo!("get_dcp_info not implemented")
    }

    async fn set_collateral_coin(&self) -> Result<ApiResponse<serde_json::Value>> {
        todo!("set_collateral_coin not implemented")
    }

    async fn set_spot_hedging(&self) -> Result<ApiResponse<serde_json::Value>> {
        todo!("set_spot_hedging not implemented")
    }

    async fn get_borrow_history(&self) -> Result<ApiResponse<serde_json::Value>> {
        todo!("get_borrow_history not implemented")
    }

    async fn batch_set_collateral_coin(&self) -> Result<ApiResponse<serde_json::Value>> {
        todo!("batch_set_collateral_coin not implemented")
    }

    async fn get_coin_greeks(&self) -> Result<ApiResponse<serde_json::Value>> {
        todo!("get_coin_greeks not implemented")
    }

    async fn get_mmp_state(&self) -> Result<ApiResponse<serde_json::Value>> {
        todo!("get_mmp_state not implemented")
    }

    async fn reset_mmp(&self) -> Result<ApiResponse<serde_json::Value>> {
        todo!("reset_mmp not implemented")
    }

    async fn set_mmp(&self) -> Result<ApiResponse<serde_json::Value>> {
        todo!("set_mmp not implemented")
    }

    async fn get_smp_group_id(&self) -> Result<ApiResponse<serde_json::Value>> {
        todo!("get_smp_group_id not implemented")
    }

    async fn get_trade_behaviour_setting(&self) -> Result<ApiResponse<serde_json::Value>> {
        todo!("get_trade_behaviour_setting not implemented")
    }

    async fn set_limit_price_behaviour(&self) -> Result<ApiResponse<serde_json::Value>> {
        todo!("set_limit_price_behaviour not implemented")
    }

    async fn repay_liability(&self) -> Result<ApiResponse<serde_json::Value>> {
        todo!("repay_liability not implemented")
    }

    async fn upgrade_to_unified_account_pro(&self) -> Result<ApiResponse<serde_json::Value>> {
        todo!("upgrade_to_unified_account_pro not implemented")
    }
}
