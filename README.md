# bybit-sdk

High-performance async Bybit HTTP API client for Rust, built for extreme throughput with advanced session pooling and zero-lock caching.

## Architecture Overview

This SDK is structured for maximum performance and scalability:

### Session Management

- **Shared Session Pool:**  
  `SessionManager` sets up a *single, high-concurrency* `reqwest` client, reused across all API clients (`BybitClient`).  
  *Best for production, bots, multi-account, or high load.*

- **Individual Sessions:**  
  If you don’t initialize `SessionManager`, each `BybitClient` builds its *own* client.  
  ⚠️ *This is much slower and uses far more system resources.*  
  *(See [benchmarks](#session-performance-benchmark) below.)*

- **Connection Pooling:**  
  The shared pool supports 2000+ concurrent connections (configurable for scale).

### Client Caching

- **TTL Cache:**  
  `ClientCache` stores constructed clients (by credentials) for 10 minutes by default.  
  Clients are re-used and dropped automatically when stale.

- **Non-blocking, Lock-Free Reads:**  
  Optimized for parallel access; fast inserts and O(1) lookups with minimal locking.

- **Auto/Manual Cleanup:**  
  Expired entries removed on demand, on access, or via background tasks.

---

## Choosing a Strategy

**If you handle many API keys (hundreds or thousands):**
- **Always:**  
  - Use `SessionManager` for a global session pool.
  - Consider `ClientCache` if you have many repeated or rotating credential sets.

- **Avoid:**  
  - Don’t create new `BybitClient`s without pooling/caching in tight loops or under load — creation is up to 270x slower per instance.

### TL;DR

> *Shared session or caching is **critical** for high-load and multi-user apps. Individual sessions are for testing or rare/small scripts only.*

---

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
bybit-sdk = "0.1"
```

---

## Quick Examples

### 1. Shared Session (RECOMMENDED for scale)

```rust
use bybit_sdk::{SessionManager, BybitClient};
use bybit_sdk::types::{AllCategory, AccountType}

#[tokio::main]
async fn main() -> bybit_sdk::Result<()> {
    // 1. Initialize the shared reqwest client for the whole process (once at startup):
    SessionManager::setup(2000);

    // 2. All BybitClients now use the shared pool
    let alice = BybitClient::new(
        Some("key1".into()),
        Some("secret1".into()),
        false, // testnet
        false, // demo
        5000,
        None,
    )?;

    let bob = BybitClient::new(
        Some("key2".into()),
        Some("secret2".into()),
        false,
        false,
        5000,
        None,
    )?;

    // Use clients as normal
    let ts = alice.get_server_time().await?;
    let tickers = bob.get_tickers(AllCategory::Spot).await?;
    let wallet = alice.get_wallet_balance(AccountType::Unified, Some("BTC")).await?;

    // 3. Gracefully close/cleanup the pool before exiting
    SessionManager::close().await;
    Ok(())
}
```

#### Performance

```
100,000 clients:    ~50 ms
Individual clients: ~13,600 ms   // (See [benchmarks](#session-performance-benchmark))
```
*A 270x speedup when using shared sessions!*

---

### 2. Individual Sessions (Not for high-load/many users)

```rust
use bybit_sdk::BybitClient;

#[tokio::main]
async fn main() -> bybit_sdk::Result<()> {
    // This creates a new reqwest client each time (slow)
    let client = BybitClient::new(
        Some("your_key".into()),
        Some("your_secret".into()),
        true,  // testnet
        false, // demo
        5000,
        None,
    )?;

    let result = client.get_server_time().await?;
    println!("{:?}", result);

    Ok(())
}
```

⚠️ *Avoid for high-frequency or multi-account scenarios! Repeated construction is slow and will benefit hugely from SessionManager or BybitClientCache.*

---

### 3. Cached Clients (Best for repeated credentials or stable credential pools)

```rust
use bybit_sdk::{BybitClientCache, BybitClient};

#[tokio::main]
async fn main() -> bybit_sdk::Result<()> {
    // Get a cached client (or create and cache if missing)
    let client = BybitClientCache::get_or_create(
        "your_key".into(),
        "your_secret".into(),
        true,  // demo
        false, // testnet
        
    )?;

    // Use the client as usual
    let ts = client.get_server_time().await?;

    // The same creds always return the same Arc<BybitClient>
    let again = BybitClientCache::get_or_create(
        "your_key".into(),
        "your_secret".into(),
        true, // demo
        false // testnet
    )?;

    assert!(Arc::ptr_eq(&client, &again));

    Ok(())
}
```

#### Cache TTL & Cleanup

```rust
use bybit_sdk::BybitClientCache;

// Set cache TTL to 30 minutes (1800 secs)
BybitClientCache::configure(1800);

// (Optional) Start automatic eviction in a background task (every 5 min)
let cleanup_task = BybitClientCache::create_cleanup_task(300);

// Manual (immediate) cleanup is also supported
let removed = BybitClientCache::cleanup_expired();
```

---

## Session Behavior Matrix

| Scenario                         | What happens?            | Session Type         |
|-----------------------------------|--------------------------|----------------------|
| `SessionManager::setup()` called  | All clients share pool   | Shared session       |
| No shared session initialized     | Each client is unique    | Individual session   |
| Using ClientCache                 | Depends on pool status   | Pooled or unique     |

---

## Session Performance Benchmark

100,000 client creation, single-threaded:

```text
Scenario               | Total Time (ms) | Per-Client (ms) | Clients/sec
-----------------------|-----------------|-----------------|-------------
Shared session         |          50.51  |        0.0005   | 1,979,796.2
Individual sessions    |       13665.66  |        0.1367   |     7,317.6
```

**Summary:**  
- Shared: **~270x faster** than repeated instantiation.
- Always use pooling and/or caching for scale!

Micro-benchmark (smaller runs):

```
BybitClient::new()        ...   17.19 ms
Cache::get_or_create()    ...    0.99 ms
SessionManager + new()    ...    0.00x ms
```

---

## Cache Performance Benchmark

10,000 clients (see `tests/test_cache.rs`):

```text
Scenario                         | Time (ms)
----------------------------------|-----------
cache_get (warm)                  |    17.00
cache_get_gather                  |    21.90
cache_get_or_create (cold/miss)   |  1163.49
cache_get_or_create_gather        |  1164.89
direct_creation_gather            |  1205.59
direct_creation                   |  1296.62
```

- *Warm cache gets are sub-millisecond per client.*
- Hot cache is best for repeat access; cold cache and fresh creation are slower.

---

## Cache Features

- **Configure TTL:** Default is 10 min, settable via `ClientCache::configure(ttl_secs)`
- **Memory Bloat Protection:** Expired clients are cleaned automatically or on schedule.
- **Lock-Free Maximized Reads:** Zero contention on lookups.
- **Background Sweeps:** Optionally run a task to purge stale clients on interval.

---

## Requirements

-   Rust >= 1.70
-   `tokio` async runtime
-   (Production: Use session pooling for best performance)

## Performance Tips

1. **Always use a shared session** (`SessionManager`) for throughput/multi-account.
2. **Use `ClientCache`** if you have repeated or rotating credentials.
3. **Tune the connection pool** (`SessionManager::setup(pool_size)`).
4. **Background cleanup** (`ClientCache::create_cleanup_task`) prevents cache bloat.

