//! High-performance cache for BybitClient instances.

use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

use crate::client::BybitClient;
use crate::error::Result;

type CacheKey = (String, String, bool, bool); // (api_key, api_secret, demo, testnet)
type CacheValue = (Arc<BybitClient>, Instant); // (client, expires_at)

static CLIENT_CACHE: Lazy<RwLock<HashMap<CacheKey, CacheValue>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));
static CACHE_LIFETIME: Lazy<RwLock<Duration>> = Lazy::new(|| RwLock::new(Duration::from_secs(600))); // 10 minutes

/// High-performance singleton cache for BybitClient instances.
///
/// Stores BybitClient instances with TTL to prevent memory leaks.
/// Equivalent to BybitClientCache in Python version.
pub struct BybitClientCache;

impl BybitClientCache {
    /// Get cached BybitClient instance or None.
    /// Note: Does not check expiration - that's handled by background cleanup.
    pub fn get(
        api_key: &str,
        api_secret: &str,
        demo: bool,
        testnet: bool,
    ) -> Option<Arc<BybitClient>> {
        let key = (api_key.to_string(), api_secret.to_string(), demo, testnet);

        if let Ok(cache) = CLIENT_CACHE.read() {
            if let Some((client, _)) = cache.get(&key) {
                return Some(Arc::clone(client));
            }
        }
        None
    }

    /// Get BybitClient from cache or create/cache it.
    pub fn get_or_create(
        api_key: String,
        api_secret: String,
        demo: bool,
        testnet: bool,
    ) -> Result<Arc<BybitClient>> {
        // Try to get from cache first
        if let Some(client) = Self::get(&api_key, &api_secret, demo, testnet) {
            return Ok(client);
        }

        // Create new client
        let client = Arc::new(BybitClient::new(
            Some(api_key.clone()),
            Some(api_secret.clone()),
            testnet,
            demo,
            5000, // recv_window
            None, // referral_id
        )?);

        // Cache it (Python stores creation time, not expiration time)
        Self::add(Arc::clone(&client), &api_key, &api_secret, demo, testnet);

        Ok(client)
    }

    /// Cache a BybitClient, overwriting any existing for key.
    pub fn add(
        client: Arc<BybitClient>,
        api_key: &str,
        api_secret: &str,
        demo: bool,
        testnet: bool,
    ) {
        let key = (api_key.to_string(), api_secret.to_string(), demo, testnet);
        let expires_at = Instant::now() + Self::lifetime();

        if let Ok(mut cache) = CLIENT_CACHE.write() {
            cache.insert(key, (client, expires_at));
        }
    }

    /// Remove all expired clients from the cache.
    /// Returns the number of entries removed.
    pub fn cleanup_expired() -> usize {
        let now = Instant::now();
        if let Ok(mut cache) = CLIENT_CACHE.write() {
            let initial_len = cache.len();
            cache.retain(|_, (_, expires_at)| *expires_at > now);
            initial_len - cache.len()
        } else {
            0
        }
    }

    /// Return the current number of entries in the cache.
    pub fn size() -> usize {
        CLIENT_CACHE.read().map(|c| c.len()).unwrap_or(0)
    }

    /// Clear all entries from cache.
    pub fn clear() {
        if let Ok(mut cache) = CLIENT_CACHE.write() {
            cache.clear();
        }
    }

    /// Get current cache lifetime
    pub fn lifetime() -> Duration {
        *CACHE_LIFETIME.read().unwrap()
    }

    /// Create background cleanup task
    pub fn create_cleanup_task(interval_seconds: u64) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(interval_seconds));
            loop {
                interval.tick().await;
                let removed = Self::cleanup_expired();
                if removed > 0 {
                    log::info!("BybitClientCache: cleaned {} entries", removed);
                }
            }
        })
    }
}
