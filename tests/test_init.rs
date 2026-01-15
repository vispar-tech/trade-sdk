//! Сравнение инициализации BybitClient тремя способами:
//! 1. Через new()
//! 2. Через BybitClientCache::get_or_create
//! 3. Через new(), но с уже активным SessionManager (shared session)
//! Выводит тайминги для каждого случая.

use bybit_sdk::cache::BybitClientCache;
use bybit_sdk::client::BybitClient;
use bybit_sdk::session::BybitSessionManager;
use std::time::Instant;

#[tokio::test]
async fn compare_client_inits() {
    let key = "test_key".to_string();
    let secret = "test_secret".to_string();

    // 1. Через new()
    let t0 = Instant::now();
    let _client1 = BybitClient::new(
        Some(key.clone()),
        Some(secret.clone()),
        false,
        false,
        5000,
        None,
    )
    .expect("client new() failed");
    let t1 = t0.elapsed().as_secs_f64() * 1000.0;

    // 2. Через BybitClientCache::get_or_create
    BybitClientCache::clear();
    let t2_start = Instant::now();
    let _client2 = BybitClientCache::get_or_create(key.clone(), secret.clone(), false, false)
        .expect("cache get_or_create failed");
    let t2 = t2_start.elapsed().as_secs_f64() * 1000.0;

    // 3. Через new(), но с уже активным SessionManager
    BybitSessionManager::close().await;
    BybitSessionManager::setup(2000);
    let t3_start = Instant::now();
    let _client3 = BybitClient::new(Some(key), Some(secret), false, false, 5000, None)
        .expect("client new() w/SessionManager failed");
    let t3 = t3_start.elapsed().as_secs_f64() * 1000.0;

    println!("BybitClient::new():           {:8.3} ms", t1);
    println!("Cache::get_or_create():       {:8.3} ms", t2);
    println!("new() with SessionManager:    {:8.3} ms", t3);

    // Cleanup
    BybitSessionManager::close().await;
}
