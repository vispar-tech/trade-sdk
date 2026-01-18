//! Benchmark test for Bybit market data endpoint: get_server_time
//! Makes NUM_REQUESTS parallel requests, times, and shows statistics.

use std::sync::Arc;
use std::time::Instant;
use trade_sdk::bybit::traits::MarketApi;
use trade_sdk::bybit::BybitClient;

const NUM_REQUESTS: usize = 10;

#[tokio::test]
async fn benchmark_bybitclient_get_server_time() {
    // Create BybitClient for testnet (no authentication required for this endpoint)
    let client = BybitClient::new(
        None,  // api_key
        None,  // api_secret
        true,  // testnet = true
        false, // demo = false
        5000,  // recv_window
        None,  // referral_id
    )
    .expect("failed to create BybitClient");

    let client = Arc::new(client);

    println!(
        "Starting benchmark: {} requests to get_server_time ...",
        NUM_REQUESTS
    );
    let start_time = Instant::now();

    let mut handles = Vec::with_capacity(NUM_REQUESTS);

    for i in 0..NUM_REQUESTS {
        let client_ref = Arc::clone(&client);
        let handle = tokio::spawn(async move {
            let idx = i;
            match client_ref.get_server_time().await {
                Ok(resp) => {
                    println!("Request {}/{}: OK", idx + 1, NUM_REQUESTS);
                    Ok(resp)
                }
                Err(e) => {
                    println!("Request {}/{}: ERROR - {}", idx + 1, NUM_REQUESTS, e);
                    Err(e)
                }
            }
        });
        handles.push(handle);
    }

    // Wait for all responses, preserving which succeeded
    let mut results = Vec::new();
    for h in handles {
        match h.await {
            Ok(Ok(resp)) => results.push(resp),
            Ok(Err(_e)) => { /* already printed error */ }
            Err(e) => println!("Task join error: {e}"),
        }
    }

    let elapsed = start_time.elapsed().as_secs_f64();

    println!("\nBenchmark completed:");
    println!("Total requests: {}", results.len());
    println!("Total time: {:.2} seconds", elapsed);
    println!(
        "Average per request: {:.4} seconds",
        elapsed / NUM_REQUESTS as f64
    );
    println!("Requests per second: {:.4}", NUM_REQUESTS as f64 / elapsed);

    if let Some(response) = results.get(0) {
        println!("\nSample response:");
        // Expecting retCode, and result: { timeSecond, timeNano }
        if let Ok(json) = serde_json::to_value(response.result.clone()) {
            if let Some(ret_code) = json.get("retCode") {
                println!("retCode: {}", ret_code);
            }
            if let Some(result) = json.get("result") {
                if let Some(ts) = result.get("timeSecond") {
                    println!("timeSecond: {}", ts);
                }
                if let Some(nano) = result.get("timeNano") {
                    println!("timeNano: {}", nano);
                }
            }
        }
    }
}
