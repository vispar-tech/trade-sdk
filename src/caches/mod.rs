/// Note: The client cache structs (such as `BybitClientsCache`, `BingxClientsCache`) implement nearly identical logic
/// and mainly differ by key types and inner client types. This leads to a significant amount of duplicated code
/// for managing client caching. In the future, these might be refactored to use generics, macros, or code generation
/// to reduce duplication and improve maintainability, possibly consolidating these caches under a single generic structure.
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

use once_cell::sync::Lazy;
use tokio::task::JoinHandle;

mod bybit;
pub use bybit::BybitClientsCache;
mod bingx;
pub use bingx::BingxClientsCache;

/// Type alias for the cache type used by all client caches.
type ClientCacheMap<K, C> = HashMap<K, (Arc<C>, Instant)>;

/// Trait for generic client caching logic, where the client type is always stored as Arc<C>.
pub trait ClientsCache<K, C>: Send + Sync + 'static
where
    K: Eq + Hash + Clone + Send + Sync + 'static,
    C: Send + Sync + 'static,
{
    /// Returns a reference to the cache (RwLock around HashMap)
    fn cache() -> &'static Lazy<RwLock<ClientCacheMap<K, C>>>;

    /// Returns a reference to the cache lifetime (RwLock around Duration)
    fn lifetime() -> &'static Lazy<RwLock<Duration>>;

    /// Update the cache expiration lifetime (in seconds).
    fn configure(lifetime_seconds: u64) {
        *Self::lifetime().write().unwrap() = Duration::from_secs(lifetime_seconds);
    }

    fn get(key: &K) -> Option<Arc<C>> {
        Self::cache().read().ok().and_then(|c| {
            c.get(key)
                .filter(|(_, exp)| exp > &Instant::now())
                .map(|(c, _)| Arc::clone(c))
        })
    }

    fn add(
        key: K,
        client: Arc<C>,
    ) {
        let expires = Instant::now() + *Self::lifetime().read().unwrap();
        if let Ok(mut cache) = Self::cache().write() {
            cache.insert(key, (client, expires));
        }
    }

    fn get_or_create<F>(
        key: K,
        create: F,
    ) -> Arc<C>
    where
        F: FnOnce() -> Arc<C>,
    {
        if let Some(c) = Self::get(&key) {
            return c;
        }
        let client = create();
        Self::add(key, Arc::clone(&client));
        client
    }

    /// Remove all expired clients from the cache.
    /// Returns the number of entries removed.
    fn cleanup_expired() -> usize {
        let now = Instant::now();
        let mut removed = 0;
        if let Ok(mut cache) = Self::cache().write() {
            let initial = cache.len();
            cache.retain(|_, (_, exp)| *exp > now);
            removed = initial - cache.len();
        }
        removed
    }

    /// Return the current number of entries in the cache.
    fn size() -> usize {
        Self::cache().read().map(|c| c.len()).unwrap_or(0)
    }

    /// Clear all entries from cache.
    fn clear() {
        if let Ok(mut cache) = Self::cache().write() {
            cache.clear();
        }
    }

    /// Create background cleanup task.
    fn create_cleanup_task(interval_seconds: u64) -> JoinHandle<()> {
        tokio::spawn(async move {
            let interval = tokio::time::Duration::from_secs(interval_seconds);
            loop {
                tokio::time::sleep(interval).await;
                let removed = Self::cleanup_expired();
                if removed > 0 {
                    log::info!(
                        "{}: cleaned {} entries",
                        std::any::type_name::<Self>(),
                        removed
                    );
                }
            }
        })
    }
}
