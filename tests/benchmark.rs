//! Simple performance benchmark for ClientsCache and SharedSessionManager.

use std::collections::BTreeMap as OrderedDict;
use std::sync::Arc;
use std::time::{Duration, Instant};

use tokio::time::sleep;
use trade_sdk::bybit::BybitClient;
use trade_sdk::SharedSessionManager;
use trade_sdk::{BybitClientsCache, ClientsCache};

const CLIENTS_COUNT: usize = 10_000;

fn make_credentials(
    i: usize,
    prefix: &str,
) -> (String, String) {
    (
        format!("{}_key_{:04}", prefix, i),
        format!("{}_secret_{:04}", prefix, i),
    )
}

fn start_timer() -> Instant {
    Instant::now()
}

fn elapsed_ms(start: Instant) -> f64 {
    start.elapsed().as_secs_f64() * 1000.0
}

struct CacheBenchmarks;

impl CacheBenchmarks {
    /// Time direct client creation without cache (sequential).
    async fn direct_creation() -> f64 {
        let mut clients = Vec::with_capacity(CLIENTS_COUNT);
        let start = start_timer();
        for i in 0..CLIENTS_COUNT {
            let (api_key, api_secret) = make_credentials(i, "direct");
            let client =
                BybitClient::new(Some(api_key), Some(api_secret), true, false, 5000, None).unwrap();
            clients.push(client);
        }
        let elapsed = elapsed_ms(start);
        // In real Python, would close clients here.
        // for client in clients { client.close().await; }
        elapsed
    }

    /// Time direct client creation without cache (parallel with gather).
    async fn direct_creation_gather() -> f64 {
        async fn create_client(i: usize) -> BybitClient {
            let (api_key, api_secret) = make_credentials(i, "direct_gather");
            BybitClient::new(Some(api_key), Some(api_secret), true, false, 5000, None).unwrap()
        }
        let start = start_timer();
        let _ = futures::future::join_all((0..CLIENTS_COUNT).map(create_client)).await;
        let elapsed = elapsed_ms(start);
        // In real Python, would close clients here.
        // for client in &clients { client.close().await; }
        elapsed
    }

    /// Time cache get_or_create (cold cache, sequential).
    async fn cache_get_or_create() -> f64 {
        BybitClientsCache::clear();
        let mut clients = Vec::with_capacity(CLIENTS_COUNT);
        let start = start_timer();
        for i in 0..CLIENTS_COUNT {
            let (api_key, api_secret) = make_credentials(i, "cache_cold");
            let client =
                BybitClientsCache::get_or_create(api_key, api_secret, true, false).unwrap();
            clients.push(client);
        }
        let elapsed = elapsed_ms(start);
        // In real Python, would close clients here.
        elapsed
    }

    /// Time cache get_or_create (cold cache, parallel with gather).
    async fn cache_get_or_create_gather() -> f64 {
        BybitClientsCache::clear();
        async fn get_or_create_client(i: usize) -> Arc<BybitClient> {
            let (api_key, api_secret) = make_credentials(i, "cache_cold_gather");
            BybitClientsCache::get_or_create(api_key, api_secret, true, false).unwrap()
        }
        let start = start_timer();
        let _ = futures::future::join_all((0..CLIENTS_COUNT).map(get_or_create_client)).await;
        let elapsed = elapsed_ms(start);
        // In real Python, would close clients here.
        elapsed
    }

    /// Time cache get (warm cache, sequential).
    async fn cache_get() -> f64 {
        BybitClientsCache::clear();
        for i in 0..CLIENTS_COUNT {
            let (api_key, api_secret) = make_credentials(i, "cache_warm");
            BybitClientsCache::get_or_create(api_key, api_secret, true, false).unwrap();
        }
        let mut clients = Vec::with_capacity(CLIENTS_COUNT);
        let start = start_timer();
        for i in 0..CLIENTS_COUNT {
            let (api_key, api_secret) = make_credentials(i, "cache_warm");
            let client = BybitClientsCache::get(&api_key, &api_secret, true, false)
                .expect(&format!("Cache miss for client {}", i));
            clients.push(client);
        }
        let elapsed = elapsed_ms(start);
        // In real Python, would close clients here.
        elapsed
    }

    /// Time cache get (warm cache, parallel with gather).
    async fn cache_get_gather() -> f64 {
        BybitClientsCache::clear();
        for i in 0..CLIENTS_COUNT {
            let (api_key, api_secret) = make_credentials(i, "cache_warm_gather");
            BybitClientsCache::get_or_create(api_key, api_secret, true, false).unwrap();
        }
        async fn get_client(i: usize) -> Arc<BybitClient> {
            let (api_key, api_secret) = make_credentials(i, "cache_warm_gather");
            BybitClientsCache::get(&api_key, &api_secret, true, false)
                .expect(&format!("Cache miss for client {}", i))
        }
        let start = start_timer();
        let _ = futures::future::join_all((0..CLIENTS_COUNT).map(get_client)).await;
        let elapsed = elapsed_ms(start);
        // In real Python, would close clients here.
        elapsed
    }
}

struct BenchmarkResultSummary;

impl BenchmarkResultSummary {
    fn print(results: &OrderedDict<String, f64>) {
        println!();
        println!("```text");
        println!(
            "Scenario                                          |     Time (ms) |   ms per client"
        );
        println!("{}", "-".repeat(70));
        let mut sorted: Vec<_> = results.iter().collect();
        sorted.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap());
        for (scenario, time) in sorted {
            let per_client = *time / CLIENTS_COUNT as f64;
            println!("{:<47} | {:12.2} | {:18.8}", scenario, time, per_client);
        }
        println!("```");
    }
}

async fn run_benchmarks_without_session_manager() -> OrderedDict<String, f64> {
    println!(
        "\nBenchmarking {} clients WITHOUT SharedSessionManager ...\n",
        CLIENTS_COUNT
    );

    let mut results = OrderedDict::new();

    results.insert(
        "direct_creation".to_string(),
        CacheBenchmarks::direct_creation().await,
    );
    results.insert(
        "direct_creation_gather".to_string(),
        CacheBenchmarks::direct_creation_gather().await,
    );
    results.insert(
        "cache_get_or_create".to_string(),
        CacheBenchmarks::cache_get_or_create().await,
    );
    results.insert(
        "cache_get_or_create_gather".to_string(),
        CacheBenchmarks::cache_get_or_create_gather().await,
    );
    results.insert("cache_get".to_string(), CacheBenchmarks::cache_get().await);
    results.insert(
        "cache_get_gather".to_string(),
        CacheBenchmarks::cache_get_gather().await,
    );

    results
}

async fn run_benchmarks_with_session_manager() -> OrderedDict<String, f64> {
    println!(
        "\nBenchmarking {} clients WITH SharedSessionManager ...\n",
        CLIENTS_COUNT
    );

    let mut results = OrderedDict::new();

    SharedSessionManager::setup(2000);

    results.insert(
        "direct_creation_with_session_manager".to_string(),
        CacheBenchmarks::direct_creation().await,
    );
    results.insert(
        "direct_creation_gather_with_session_manager".to_string(),
        CacheBenchmarks::direct_creation_gather().await,
    );
    results.insert(
        "cache_get_or_create_with_session_manager".to_string(),
        CacheBenchmarks::cache_get_or_create().await,
    );
    results.insert(
        "cache_get_or_create_gather_with_session_manager".to_string(),
        CacheBenchmarks::cache_get_or_create_gather().await,
    );
    results.insert(
        "cache_get_with_session_manager".to_string(),
        CacheBenchmarks::cache_get().await,
    );
    results.insert(
        "cache_get_gather_with_session_manager".to_string(),
        CacheBenchmarks::cache_get_gather().await,
    );

    results
}

#[tokio::test]
async fn test_cache_performance_benchmark() {
    println!("ClientsCache and SharedSessionManager Performance Benchmark");
    println!("{}", "=".repeat(60));
    // Phase 1: WITHOUT SharedSessionManager
    let results_without_shared = run_benchmarks_without_session_manager().await;
    BenchmarkResultSummary::print(&results_without_shared);

    // Cleanup: clear cache and close SharedSessionManager (if needed)
    BybitClientsCache::clear();
    SharedSessionManager::close().await;
    sleep(Duration::from_millis(100)).await;

    // Phase 2: WITH SharedSessionManager
    let results_with_shared = run_benchmarks_with_session_manager().await;
    BenchmarkResultSummary::print(&results_with_shared);

    // Final cleanup
    BybitClientsCache::clear();
    SharedSessionManager::close().await;
}
