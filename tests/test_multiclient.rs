//! Benchmark test for multiple Bybit clients, each making several requests to get_server_time.

use std::sync::Arc;
use std::time::Instant;
use trade_sdk::bybit::traits::MarketApi;
use trade_sdk::bybit::BybitClient;

const NUM_CLIENTS: usize = 10;
const REQUESTS_PER_CLIENT: usize = 3;

#[tokio::test]
async fn benchmark_bybitclient_multiclient_get_server_time() {
    // Number of clients and requests per client
    println!(
        "Starting multiclient benchmark: {} clients making {} requests each...",
        NUM_CLIENTS, REQUESTS_PER_CLIENT
    );

    // Create BybitClient instances for testnet (no auth required for this endpoint)
    let mut clients = Vec::with_capacity(NUM_CLIENTS);
    for _ in 0..NUM_CLIENTS {
        let client = BybitClient::new(
            None,  // api_key
            None,  // api_secret
            true,  // testnet = true
            false, // demo = false
            5000,  // recv_window
            None,  // referral_id
        )
        .expect("failed to create BybitClient");
        clients.push(Arc::new(client));
    }

    let start_time = Instant::now();
    let mut all_handles = Vec::with_capacity(NUM_CLIENTS * REQUESTS_PER_CLIENT);

    // For each client, spawn a separate async task for its requests
    for (client_idx, client) in clients.iter().enumerate() {
        let client = Arc::clone(client);
        let client_idx = client_idx; // For move in async block
        let handle = tokio::spawn(async move {
            let mut results = Vec::with_capacity(REQUESTS_PER_CLIENT);
            let mut client_handles = Vec::new();
            for request_idx in 0..REQUESTS_PER_CLIENT {
                let client_ref = Arc::clone(&client);
                let client_idx = client_idx;
                client_handles.push(tokio::spawn(async move {
                    match client_ref.get_server_time().await {
                        Ok(resp) => {
                            println!(
                                "Client {}/{}: Request {}/{}: OK",
                                client_idx + 1,
                                NUM_CLIENTS,
                                request_idx + 1,
                                REQUESTS_PER_CLIENT
                            );
                            Ok(resp)
                        }
                        Err(e) => {
                            println!(
                                "Client {}/{}: Request {}/{}: ERROR - {}",
                                client_idx + 1,
                                NUM_CLIENTS,
                                request_idx + 1,
                                REQUESTS_PER_CLIENT,
                                e
                            );
                            Err(e)
                        }
                    }
                }));
            }
            // Await this client's requests (in parallel within client)
            for h in client_handles {
                if let Ok(Ok(resp)) = h.await {
                    results.push(resp);
                }
                // Errors are already printed
            }
            results
        });
        all_handles.push(handle);
    }

    // Collect all results from all clients
    let mut all_results = Vec::with_capacity(NUM_CLIENTS * REQUESTS_PER_CLIENT);
    for h in all_handles {
        match h.await {
            Ok(results) => {
                all_results.extend(results);
            }
            Err(e) => {
                println!("Client task join error: {e}");
            }
        }
    }

    let elapsed = start_time.elapsed().as_secs_f64();
    let total_requests = NUM_CLIENTS * REQUESTS_PER_CLIENT;

    println!("\nMulticlient benchmark completed:");
    println!("Total requests: {}", all_results.len());
    println!("Total time: {:.2} seconds", elapsed);
    println!(
        "Average per request: {:.4} seconds",
        elapsed / total_requests as f64
    );
    println!(
        "Requests per second: {:.4}",
        total_requests as f64 / elapsed
    );

    if let Some(response) = all_results.get(0) {
        println!("\nSample response:");
        // Try to extract retCode/result/timeSecond/timeNano for display
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
