//! Simple performance benchmark for BybitClientCache.
//!
//! Equivalent to test_cache.py in Python reference.

use std::sync::Once;
use std::time::Instant;

static INIT_LOGGER: Once = Once::new();

/// Initialize logger once for all tests.
/// Safe to call multiple times - logger will be initialized only once.
pub fn init_test_logger() {
    INIT_LOGGER.call_once(|| {
        let _ = env_logger::builder().is_test(true).try_init();
    });
}

/// Generate unique API credentials for testing.
fn make_credentials(
    i: usize,
    prefix: &str,
) -> (String, String) {
    (
        format!("{}_key_{:04}", prefix, i),
        format!("{}_secret_{:04}", prefix, i),
    )
}

#[inline]
fn start_timer() -> Instant {
    Instant::now()
}

#[inline]
fn elapsed_ms(start: Instant) -> f64 {
    start.elapsed().as_secs_f64() * 1000.0
}

/// Time direct client creation without cache (sequential).
async fn benchmark_direct_creation(client_count: usize) -> f64 {
    use bybit_sdk::client::BybitClient;
    let mut clients = Vec::with_capacity(client_count);

    let start = start_timer();
    for i in 0..client_count {
        let (api_key, api_secret) = make_credentials(i, "direct");
        let client =
            BybitClient::new(Some(api_key), Some(api_secret), true, false, 5000, None).unwrap();
        clients.push(client);
    }
    let elapsed = elapsed_ms(start);
    println!("Direct creation (sequential): {:.2} ms", elapsed);
    drop(clients);
    elapsed
}

/// Time direct client creation without cache (parallel with gather).
async fn benchmark_direct_creation_gather(client_count: usize) -> f64 {
    use bybit_sdk::client::BybitClient;
    use futures::future::join_all;

    async fn create_client(i: usize) -> BybitClient {
        let (api_key, api_secret) = make_credentials(i, "direct_gather");
        BybitClient::new(Some(api_key), Some(api_secret), true, false, 5000, None).unwrap()
    }

    let start = start_timer();
    let clients: Vec<BybitClient> = join_all((0..client_count).map(|i| create_client(i))).await;
    let elapsed = elapsed_ms(start);
    println!("Direct creation (gather): {:.2} ms", elapsed);
    drop(clients);
    elapsed
}

/// Time cache get_or_create (cold cache, sequential).
async fn benchmark_cache_get_or_create(client_count: usize) -> f64 {
    use bybit_sdk::cache::BybitClientCache;
    BybitClientCache::clear();
    let mut clients = Vec::with_capacity(client_count);

    let start = start_timer();
    for i in 0..client_count {
        let (api_key, api_secret) = make_credentials(i, "cache_cold");
        let client = BybitClientCache::get_or_create(api_key, api_secret, false, false).unwrap();
        clients.push(client);
    }
    let elapsed = elapsed_ms(start);
    println!("Cache get_or_create (cold, sequential): {:.2} ms", elapsed);
    drop(clients);
    elapsed
}

/// Time cache get_or_create (cold cache, parallel with gather).
async fn benchmark_cache_get_or_create_gather(client_count: usize) -> f64 {
    use bybit_sdk::cache::BybitClientCache;
    use futures::future::join_all;
    BybitClientCache::clear();

    async fn get_or_create_client(i: usize) -> std::sync::Arc<bybit_sdk::client::BybitClient> {
        let (api_key, api_secret) = make_credentials(i, "cache_cold_gather");
        BybitClientCache::get_or_create(api_key, api_secret, false, false).unwrap()
    }

    let start = start_timer();
    let _ = join_all((0..client_count).map(|i| get_or_create_client(i))).await;
    let elapsed = elapsed_ms(start);
    println!("Cache get_or_create (cold, gather): {:.2} ms", elapsed);
    elapsed
}

/// Time cache get (warm cache, sequential).
async fn benchmark_cache_get(client_count: usize) -> f64 {
    use bybit_sdk::cache::BybitClientCache;
    BybitClientCache::clear();

    // Pre-populate cache
    for i in 0..client_count {
        let (api_key, api_secret) = make_credentials(i, "cache_warm");
        BybitClientCache::get_or_create(api_key, api_secret, false, false).unwrap();
    }

    let mut clients = Vec::with_capacity(client_count);

    let start = start_timer();
    for i in 0..client_count {
        let (api_key, api_secret) = make_credentials(i, "cache_warm");
        let client = BybitClientCache::get(&api_key, &api_secret, false, false)
            .expect(&format!("Cache miss for client {}", i));
        clients.push(client);
    }
    let elapsed = elapsed_ms(start);
    println!("Cache get (warm, sequential): {:.2} ms", elapsed);
    drop(clients);
    elapsed
}

/// Time cache get (warm cache, parallel with gather).
async fn benchmark_cache_get_gather(client_count: usize) -> f64 {
    use bybit_sdk::cache::BybitClientCache;
    use futures::future::join_all;
    BybitClientCache::clear();

    // Pre-populate cache
    for i in 0..client_count {
        let (api_key, api_secret) = make_credentials(i, "cache_warm_gather");
        BybitClientCache::get_or_create(api_key, api_secret, false, false).unwrap();
    }

    async fn get_client(i: usize) -> std::sync::Arc<bybit_sdk::client::BybitClient> {
        let (api_key, api_secret) = make_credentials(i, "cache_warm_gather");
        BybitClientCache::get(&api_key, &api_secret, false, false)
            .expect(&format!("Cache miss for client {}", i))
    }

    let start = start_timer();
    let _ = join_all((0..client_count).map(|i| get_client(i))).await;
    let elapsed = elapsed_ms(start);
    println!("Cache get (warm, gather): {:.2} ms", elapsed);
    elapsed
}

/// Run all benchmarks and return results.
async fn run_benchmarks(client_count: usize) -> std::collections::BTreeMap<String, f64> {
    println!("\nBenchmarking {} clients...\n", client_count);

    let mut results = std::collections::BTreeMap::new();

    println!("Testing direct creation (sequential)...");
    results.insert(
        "direct_creation".to_string(),
        benchmark_direct_creation(client_count).await,
    );

    println!("Testing direct creation (gather)...");
    results.insert(
        "direct_creation_gather".to_string(),
        benchmark_direct_creation_gather(client_count).await,
    );

    println!("Testing cache get_or_create (cold, sequential)...");
    results.insert(
        "cache_get_or_create".to_string(),
        benchmark_cache_get_or_create(client_count).await,
    );

    println!("Testing cache get_or_create (cold, gather)...");
    results.insert(
        "cache_get_or_create_gather".to_string(),
        benchmark_cache_get_or_create_gather(client_count).await,
    );

    println!("Testing cache get (warm, sequential)...");
    results.insert(
        "cache_get".to_string(),
        benchmark_cache_get(client_count).await,
    );

    println!("Testing cache get (warm, gather)...");
    results.insert(
        "cache_get_gather".to_string(),
        benchmark_cache_get_gather(client_count).await,
    );

    results
}

/// Print summary sorted by best (fastest) timings.
fn print_best_summary(results: &std::collections::BTreeMap<String, f64>) {
    println!();
    println!("```text");
    println!("Scenario                                |    Time (ms)");
    println!("{}", "-".repeat(55));

    let mut sorted: Vec<_> = results.iter().collect();
    sorted.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap());

    for (scenario, time) in sorted {
        println!("{:<36} | {:12.2}", scenario, time);
    }
    println!("```");
}

#[tokio::test]
async fn test_cache_performance_benchmark() {
    init_test_logger();
    let client_count = 10_000;

    println!("BybitClientCache Performance Benchmark");
    println!("{}", "=".repeat(40));

    let results = run_benchmarks(client_count).await;
    print_best_summary(&results);

    // Cleanup
    bybit_sdk::cache::BybitClientCache::clear();
}
