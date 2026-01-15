//! Test for BybitSessionManager performance.

use std::collections::HashMap;
use std::time::Instant;
use tokio::time::{sleep, Duration};

use bybit_sdk::http::BybitHttpClient;
use bybit_sdk::session::BybitSessionManager;

/// Cleanup helper to close session manager and let resources settle.
async fn cleanup_between_scenarios() {
    BybitSessionManager::close().await;
    sleep(Duration::from_millis(100)).await;
    // Rust does not require explicit garbage collection.
}

/// Compare client creation with and without session manager.
/// Returns timing results (ms) for each scenario.
async fn benchmark_session_manager_scenarios(client_count: usize) -> HashMap<String, f64> {
    let mut results: HashMap<String, f64> = HashMap::from([
        ("individual_sessions_10000_clients".to_string(), 0.0),
        ("shared_session_10000_clients".to_string(), 0.0),
    ]);

    println!(
        "Running session manager benchmark with {} clients...",
        client_count
    );

    // SCENARIO 1: Each client has its own session
    println!(
        "Scenario 1: Creating {} clients with individual sessions...",
        client_count
    );
    cleanup_between_scenarios().await;

    let start_time = Instant::now();
    let mut clients_without_session: Vec<BybitHttpClient> = Vec::with_capacity(client_count);
    for i in 0..client_count {
        let client = BybitHttpClient::new(
            Some(format!("individual_key_{:04}", i)),
            Some(format!("individual_secret_{:04}", i)),
            false,
            false,
            5000,
            None,
        );
        clients_without_session.push(client);
    }
    let elapsed_ms = start_time.elapsed().as_secs_f64() * 1000.0;
    results.insert("individual_sessions_10000_clients".to_string(), elapsed_ms);

    // Verification: Count how many clients use individual sessions
    let individual_count = clients_without_session
        .iter()
        .filter(|c| !c.uses_shared_session())
        .count();
    println!(
        "Individual session verification: {}/{} clients use individual sessions",
        individual_count, client_count
    );

    // Cleanup individual sessions
    drop(clients_without_session);

    // SCENARIO 2: Use shared session manager
    println!(
        "Scenario 2: Creating {} clients with shared session...",
        client_count
    );
    cleanup_between_scenarios().await;
    println!("Setting up session manager...");
    BybitSessionManager::setup(2000);

    let start_time = Instant::now();
    let mut clients_with_session: Vec<BybitHttpClient> = Vec::with_capacity(client_count);
    for i in 0..client_count {
        let client = BybitHttpClient::new(
            Some(format!("shared_key_{:04}", i)),
            Some(format!("shared_secret_{:04}", i)),
            false,
            false,
            5000,
            None,
        );
        clients_with_session.push(client);
    }
    let elapsed_ms = start_time.elapsed().as_secs_f64() * 1000.0;
    results.insert("shared_session_10000_clients".to_string(), elapsed_ms);

    // Verification: Count how many clients use shared session
    let shared_count = clients_with_session
        .iter()
        .filter(|c| c.uses_shared_session())
        .count();
    println!(
        "Shared session verification: {}/{} clients use shared session",
        shared_count, client_count
    );

    // Final cleanup
    cleanup_between_scenarios().await;

    results
}

/// Print formatted session manager benchmark summary.
fn print_session_manager_summary(
    results: &HashMap<String, f64>,
    client_count: usize,
) {
    println!("\n{}", "=".repeat(80));
    println!("SESSION MANAGER PERFORMANCE COMPARISON");
    println!("{}", "=".repeat(80));

    let indv_ms = results["individual_sessions_10000_clients"];
    let shared_ms = results["shared_session_10000_clients"];

    let scenario_data = {
        let mut data = vec![
            (
                "Individual sessions",
                indv_ms,
                indv_ms / client_count as f64,
                if indv_ms > 0.0 {
                    1000.0 * (client_count as f64 / indv_ms)
                } else {
                    0.0
                },
            ),
            (
                "Shared session",
                shared_ms,
                shared_ms / client_count as f64,
                if shared_ms > 0.0 {
                    1000.0 * (client_count as f64 / shared_ms)
                } else {
                    0.0
                },
            ),
        ];
        data.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        data
    };

    println!("Total clients: {}", client_count);
    println!("\n```text");
    println!("Scenario               | Total Time (ms) | Time per client (ms) | Clients/sec");
    println!("{:-<85}", "");
    for (scenario, total_time, time_per_client, clients_per_sec) in &scenario_data {
        println!(
            "{:<22} | {:14.2} | {:18.4} | {:10.1}",
            scenario, total_time, time_per_client, clients_per_sec
        );
    }
    println!("```");
    println!();

    println!("PERFORMANCE ANALYSIS:");
    println!("{}", "-".repeat(30));
    if indv_ms > 0.0 && shared_ms > 0.0 {
        let improvement = if indv_ms > 0.0 {
            (indv_ms - shared_ms) / indv_ms * 100.0
        } else {
            0.0
        };
        let speedup = if shared_ms > 0.0 {
            indv_ms / shared_ms
        } else {
            0.0
        };
        println!("Shared session vs Individual sessions:");
        println!("  - Time difference: {:.2} ms", indv_ms - shared_ms);
        println!("  - Performance improvement: {:.1}%", improvement);
        println!("  - Shared session is {:.1}x faster", speedup);
        println!();
        println!("Conclusion:");
        if speedup >= 2.0 {
            println!("  ✓ Shared session provides significant performance benefit");
            println!("    for high-frequency client creation");
        } else if speedup >= 1.5 {
            println!("  ✓ Shared session provides good performance benefit");
        } else if speedup >= 1.1 {
            println!("  ~ Shared session provides moderate performance benefit");
        } else {
            println!("  ⚠ Shared session benefit is minimal - consider individual");
            println!("    sessions for low-frequency use");
        }
    }
}

/// Run session manager benchmark tests.
#[tokio::test]
async fn main() {
    println!("BybitSessionManager Performance Benchmark");
    println!("Comparing individual sessions vs shared session");

    let client_count = 100_000;
    let results = benchmark_session_manager_scenarios(client_count).await;
    print_session_manager_summary(&results, client_count);
}
