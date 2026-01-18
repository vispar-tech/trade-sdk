//! High-performance session management for trading API clients.

use once_cell::sync::Lazy;
use reqwest::Client;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};
use std::time::Duration;

static SHARED_SESSION_MANAGER: Lazy<RwLock<Option<SharedSessionManager>>> = Lazy::new(|| RwLock::new(None));
static SESSION_INITIALIZED: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));

/// Manager for shared reqwest client with high-performance connection pool.
///
/// Equivalent to SharedSessionManager in Python version.
pub struct SharedSessionManager {
    client: Arc<Client>,
    max_connections: usize,
}

impl SharedSessionManager {
    /// Initialize shared session with high-performance connection pool.
    ///
    /// Call this once at application startup.
    ///
    /// # Arguments
    /// * `max_connections` - Maximum number of connections in pool (default 2000)
    pub fn setup(max_connections: usize) {
        // Fast atomic check first
        if SESSION_INITIALIZED.load(Ordering::Acquire) {
            log::warn!("Session already initialized - skipping setup");
            return;
        }

        let mut manager = SHARED_SESSION_MANAGER.write().unwrap();

        if manager.is_some() {
            log::warn!("Session already initialized - skipping setup");
            return;
        }

        log::info!(
            "Initializing shared session with {} max connections",
            max_connections
        );

        // Create client equivalent to aiohttp.ClientSession with TCPConnector
        let client = Client::builder()
            // Connection pool settings - equivalent to aiohttp connector limits
            .pool_max_idle_per_host(max_connections / 2) // limit_per_host = max_connections // 2
            .pool_idle_timeout(Duration::from_secs(60)) // keepalive_timeout=60
            .tcp_keepalive(Duration::from_secs(60)) // Keep connections alive
            .tcp_nodelay(true) // Disable Nagle's algorithm for lower latency
            // Timeout settings
            .timeout(Duration::from_secs(30))
            // HTTP/1.1 for compatibility with aiohttp
            .http1_only()
            .user_agent("trade-sdk/0.1.0")
            // Default headers - same as Python
            .default_headers({
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert("Content-Type", "application/json".parse().unwrap());
                headers.insert("Accept", "application/json".parse().unwrap());
                // Add keep-alive header explicitly
                headers.insert("Connection", "keep-alive".parse().unwrap());
                headers
            })
            .build()
            .expect("Failed to create HTTP client");

        *manager = Some(SharedSessionManager {
            client: Arc::new(client),
            max_connections,
        });

        // Mark as initialized atomically
        SESSION_INITIALIZED.store(true, Ordering::Release);

        log::info!("✅ Shared session initialized with maximum performance settings");
    }

    /// Check if shared session is initialized and active.
    /// Ultra-fast atomic check.
    pub fn is_initialized() -> bool {
        SESSION_INITIALIZED.load(Ordering::Acquire)
    }

    /// Get shared client with zero-copy Arc cloning.
    /// Ultra-fast read operation using RwLock - no blocking for concurrent reads.
    pub fn get_client() -> Arc<Client> {
        // Fast atomic check first
        if !SESSION_INITIALIZED.load(Ordering::Acquire) {
            panic!("Session not initialized. Call SessionManager::setup() first.");
        }

        // RwLock read - allows multiple concurrent readers
        if let Ok(manager) = SHARED_SESSION_MANAGER.read() {
            if let Some(ref session) = *manager {
                return Arc::clone(&session.client);
            }
        }

        // Fallback with write lock if read failed
        SHARED_SESSION_MANAGER
            .write()
            .unwrap()
            .as_ref()
            .expect("Session not initialized. Call SessionManager::setup() first.")
            .client
            .clone()
    }

    /// Close the shared session gracefully.
    /// Call this at application shutdown.
    pub async fn close() {
        // Atomic flag first
        if !SESSION_INITIALIZED.swap(false, Ordering::AcqRel) {
            log::debug!("Session already closed or not initialized");
            return;
        }

        // Scope for the manager lock to ensure it's dropped before await
        let should_wait = {
            let mut manager = SHARED_SESSION_MANAGER.write().unwrap();
            manager.take().is_some()
        };
        if should_wait {
            log::info!("Closing shared session gracefully");
            // Give time for pending requests to complete
            tokio::time::sleep(Duration::from_millis(200)).await;
            log::info!("✅ Shared session closed successfully");
        }
    }

    /// Get maximum connections setting
    pub fn max_connections() -> usize {
        if let Ok(manager) = SHARED_SESSION_MANAGER.read() {
            if let Some(ref session) = *manager {
                return session.max_connections;
            }
        }
        0
    }
}
