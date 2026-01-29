use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

use super::super::caches::ClientsCache;
use crate::bingx::BingxClient;
use crate::error::Result;

/// Type alias for the unique cache key for Bingx clients.
/// Format: (api_key, api_secret, demo, testnet)
pub type BingxCacheKey = (String, String, bool, bool);

/// Type alias for the type stored in the Bingx client cache.
type BingxCacheValue = (Arc<BingxClient>, Instant);

/// Global Bingx client cache (thread-safe, shared by all).
static BINGX_CACHE: Lazy<RwLock<HashMap<BingxCacheKey, BingxCacheValue>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

/// Default cache entry lifetime for Bingx client cache.
static BINGX_CACHE_LIFETIME: Lazy<RwLock<Duration>> =
    Lazy::new(|| RwLock::new(Duration::from_secs(600))); // 10 minutes

/// Cache for BingxClient connections, keyed by API credentials and flags.
pub struct BingxClientsCache;

impl ClientsCache<BingxCacheKey, BingxClient> for BingxClientsCache {
    /// Returns a reference to the global cache storage.
    fn cache() -> &'static Lazy<RwLock<HashMap<BingxCacheKey, (Arc<BingxClient>, Instant)>>> {
        &BINGX_CACHE
    }

    /// Returns a reference to the cache lifetime duration.
    fn lifetime() -> &'static Lazy<RwLock<Duration>> {
        &BINGX_CACHE_LIFETIME
    }
}

/// Constructs a key for cache lookup or storage.
///
/// # Arguments
/// * `api_key` - API key as Into<String>
/// * `api_secret` - API secret as Into<String>
/// * `testnet` - Use Bingx testnet
/// * `demo` - Use demo mode
#[inline]
fn make_key(
    api_key: impl Into<String>,
    api_secret: impl Into<String>,
    testnet: bool,
    demo: bool,
) -> (String, String, bool, bool) {
    (api_key.into(), api_secret.into(), demo, testnet)
}

impl BingxClientsCache {
    /// Fetch a BingxClient from the cache, or create and insert one if missing.
    ///
    /// # Arguments
    /// * `api_key` - The Bingx API key (consumed as String)
    /// * `api_secret` - The Bingx API secret (consumed as String)
    /// * `testnet` - Whether to use Bingx testnet API
    /// * `demo` - Whether to use demo mode for client
    ///
    /// # Returns
    /// * Ok(Arc<BingxClient>) - Shared reference to the client
    /// * Err(crate::error::Error) - If client creation fails
    pub fn get_or_create(
        api_key: impl Into<String>,
        api_secret: impl Into<String>,
        testnet: bool,
        demo: bool,
    ) -> Result<Arc<BingxClient>> {
        let key = make_key(api_key, api_secret, demo, testnet);

        if let Some(client) = <Self as ClientsCache<BingxCacheKey, BingxClient>>::get(&key) {
            return Ok(client);
        }

        let client = Arc::new(BingxClient::new(
            Some(key.0.clone()),
            Some(key.1.clone()),
            demo,
            5000,
        )?);

        <Self as ClientsCache<BingxCacheKey, BingxClient>>::add(key, Arc::clone(&client));

        Ok(client)
    }

    /// Fetch a BingxClient from the cache by credentials and flags.
    ///
    /// # Arguments
    /// * `api_key` - API key as &str
    /// * `api_secret` - API secret as &str
    /// * `testnet` - Use Bingx testnet
    /// * `demo` - Use demo mode
    pub fn get(
        api_key: &str,
        api_secret: &str,
        testnet: bool,
        demo: bool,
    ) -> Option<Arc<BingxClient>> {
        let key = make_key(api_key, api_secret, demo, testnet);
        <Self as ClientsCache<BingxCacheKey, BingxClient>>::get(&key)
    }

    /// Add a BingxClient to the cache with the given credentials and flags.
    ///
    /// # Arguments
    /// * `client` - Arc-wrapped BingxClient to insert
    /// * `api_key` - API key as &str
    /// * `api_secret` - API secret as &str
    /// * `testnet` - Use Bingx testnet
    /// * `demo` - Use demo mode
    pub fn add(
        client: Arc<BingxClient>,
        api_key: &str,
        api_secret: &str,
        testnet: bool,
        demo: bool,
    ) {
        let key = make_key(api_key, api_secret, demo, testnet);
        <Self as ClientsCache<BingxCacheKey, BingxClient>>::add(key, client);
    }
}
