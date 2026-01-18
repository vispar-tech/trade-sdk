use crate::bingx::traits::account::FundApi;
use crate::bingx::BingxClient;
use async_trait::async_trait;

#[async_trait]
impl FundApi for BingxClient {}
