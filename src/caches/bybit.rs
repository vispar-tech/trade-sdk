use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

use super::super::caches::ClientsCache;
use crate::bybit::BybitClient;
use crate::error::Result;

/// Type alias for the unique cache key for Bybit clients.
/// Format: (api_key, api_secret, demo, testnet)
pub type BybitCacheKey = (String, String, bool, bool);

/// Type alias for the type stored in the Bybit client cache.
type BybitCacheValue = (Arc<BybitClient>, Instant);

/// Global Bybit client cache (thread-safe, shared by all).
static BYBIT_CACHE: Lazy<RwLock<HashMap<BybitCacheKey, BybitCacheValue>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

/// Default cache entry lifetime for Bybit client cache.
static BYBIT_CACHE_LIFETIME: Lazy<RwLock<Duration>> =
    Lazy::new(|| RwLock::new(Duration::from_secs(600))); // 10 minutes

/// Cache for BybitClient connections, keyed by API credentials and flags.
pub struct BybitClientsCache;

impl ClientsCache<BybitCacheKey, BybitClient> for BybitClientsCache {
    /// Returns a reference to the global cache storage.
    fn cache() -> &'static Lazy<RwLock<HashMap<BybitCacheKey, (Arc<BybitClient>, Instant)>>> {
        &BYBIT_CACHE
    }

    /// Returns a reference to the cache lifetime duration.
    fn lifetime() -> &'static Lazy<RwLock<Duration>> {
        &BYBIT_CACHE_LIFETIME
    }
}

/// Constructs a key for cache lookup or storage.
///
/// # Arguments
/// * `api_key` - API key as Into<String>
/// * `api_secret` - API secret as Into<String>
/// * `testnet` - Use Bybit testnet
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

impl BybitClientsCache {
    /// Fetch a BybitClient from the cache, or create and insert one if missing.
    ///
    /// # Arguments
    /// * `api_key` - The Bybit API key (consumed as String)
    /// * `api_secret` - The Bybit API secret (consumed as String)
    /// * `testnet` - Whether to use Bybit testnet API
    /// * `demo` - Whether to use demo mode for client
    ///
    /// # Returns
    /// * Ok(Arc<BybitClient>) - Shared reference to the client
    /// * Err(crate::error::Error) - If client creation fails
    pub fn get_or_create(
        api_key: impl Into<String>,
        api_secret: impl Into<String>,
        testnet: bool,
        demo: bool,
    ) -> Result<Arc<BybitClient>> {
        let key = make_key(api_key, api_secret, demo, testnet);

        if let Some(client) = <Self as ClientsCache<BybitCacheKey, BybitClient>>::get(&key) {
            return Ok(client);
        }

        let client = Arc::new(BybitClient::new(
            Some(key.0.clone()),
            Some(key.1.clone()),
            testnet,
            demo,
            5000,
            None,
        )?);

        <Self as ClientsCache<BybitCacheKey, BybitClient>>::add(key, Arc::clone(&client));

        Ok(client)
    }

    /// Fetch a BybitClient from the cache by credentials and flags.
    ///
    /// # Arguments
    /// * `api_key` - API key as &str
    /// * `api_secret` - API secret as &str
    /// * `testnet` - Use Bybit testnet
    /// * `demo` - Use demo mode
    pub fn get(
        api_key: &str,
        api_secret: &str,
        testnet: bool,
        demo: bool,
    ) -> Option<Arc<BybitClient>> {
        let key = make_key(api_key, api_secret, demo, testnet);
        <Self as ClientsCache<BybitCacheKey, BybitClient>>::get(&key)
    }

    /// Add a BybitClient to the cache with the given credentials and flags.
    ///
    /// # Arguments
    /// * `client` - Arc-wrapped BybitClient to insert
    /// * `api_key` - API key as &str
    /// * `api_secret` - API secret as &str
    /// * `testnet` - Use Bybit testnet
    /// * `demo` - Use demo mode
    pub fn add(
        client: Arc<BybitClient>,
        api_key: &str,
        api_secret: &str,
        testnet: bool,
        demo: bool,
    ) {
        let key = make_key(api_key, api_secret, demo, testnet);
        <Self as ClientsCache<BybitCacheKey, BybitClient>>::add(key, client);
    }
}
