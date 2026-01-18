use crate::bingx::traits::mfutures::MarketApi;

use crate::bingx::BingxClient;

use async_trait::async_trait;

#[async_trait]
impl MarketApi for BingxClient {}
