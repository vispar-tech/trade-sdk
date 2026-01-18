use crate::{bingx::types::ApiResponse, error::Result};

use async_trait::async_trait;

#[async_trait]
pub trait CommonApi {
    /// Gets the BingX server time.
    ///
    /// # Returns
    ///
    /// Returns a [`GenericResponse`] containing the server time response from the BingX API.
    ///
    /// # See also
    ///
    /// [BingX API Documentation - Basic Information](https://bingx-api.github.io/docs-v3/#/en/Quick%20Start/Basic%20Information)
    async fn get_server_time(&self) -> Result<ApiResponse<serde_json::Value>>;
}
