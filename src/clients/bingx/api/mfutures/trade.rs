use crate::bingx::traits::mfutures::TradeApi;
use crate::bingx::BingxClient;
use async_trait::async_trait;

#[async_trait]
impl TradeApi for BingxClient {}
