use crate::bingx::traits::account::WalletApi;
use crate::bingx::BingxClient;
use async_trait::async_trait;

#[async_trait]
impl WalletApi for BingxClient {}
