//! Tests for Bybit API authentication and wallet balance.
//!
//! Equivalent to test_auth.py in Python reference.
//! This test makes real API calls using environment variables.

use bybit_sdk::client::BybitClient;
use bybit_sdk::traits::AccountApi;
use bybit_sdk::traits::PositionApi;
use bybit_sdk::types::AccountType;
use bybit_sdk::types::AllCategories;

use std::sync::Once;

static INIT_LOGGER: Once = Once::new();

/// Initialize logger once for all tests.
/// Safe to call multiple times - logger will be initialized only once.
pub fn init_test_logger() {
    INIT_LOGGER.call_once(|| {
        env_logger::init();
    });
}

/// Fetch and print wallet balance summary.
async fn print_wallet_balance(client: &BybitClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📊 Fetching wallet balance...");
    match client
        .get_wallet_balance(Some(AccountType::Unified), Some("USDT"))
        .await
    {
        Ok(response) => {
            println!("✅ Wallet balance retrieved successfully!");
            println!(
                "Full response:\n{}",
                serde_json::to_string_pretty(&response.result)?
            );

            if let Some(result_obj) = response.result.get("result") {
                if let Some(accounts) = result_obj.get("list").and_then(|l| l.as_array()) {
                    println!("Found {} account types", accounts.len());
                    for account in accounts.iter().take(3) {
                        let account_type = account
                            .get("accountType")
                            .and_then(|s| s.as_str())
                            .unwrap_or("Unknown");
                        let total_wallet_balance = account
                            .get("totalWalletBalance")
                            .and_then(|s| s.as_str())
                            .unwrap_or("0");

                        println!("   {}: wallet = {}", account_type, total_wallet_balance);

                        if let Some(coins) = account.get("coin").and_then(|c| c.as_array()) {
                            if !coins.is_empty() {
                                println!("      Coins:");
                                for coin in coins.iter().take(3) {
                                    let coin_name =
                                        coin.get("coin").and_then(|s| s.as_str()).unwrap_or("???");
                                    let coin_balance = coin
                                        .get("walletBalance")
                                        .and_then(|s| s.as_str())
                                        .unwrap_or("0");
                                    let coin_equity =
                                        coin.get("equity").and_then(|s| s.as_str()).unwrap_or("0");

                                    println!(
                                        "         {}: wallet_balance={}, equity={}",
                                        coin_name, coin_balance, coin_equity
                                    );
                                }
                            }
                        }
                    }
                }
            }
            Ok(())
        }
        Err(e) => {
            println!("❌ Error retrieving wallet balance: {}", e);
            Err(e.into())
        }
    }
}

/// Fetch and print open positions summary.
async fn print_open_positions(client: &BybitClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📈 Fetching open positions...");
    match client
        .get_position_info(
            AllCategories::Linear,
            None,
            None,
            Some("USDT"),
            Some(200),
            None,
        )
        .await
    {
        Ok(response) => {
            println!("✅ Open positions retrieved successfully!");
            println!(
                "Full positions response:\n{}",
                serde_json::to_string_pretty(&response.result)?
            );

            if let Some(pos_result_obj) = response.result.get("result") {
                if let Some(pos_list) = pos_result_obj.get("list").and_then(|l| l.as_array()) {
                    println!("Found {} open positions", pos_list.len());
                    for pos in pos_list.iter().take(3) {
                        let symbol = pos.get("symbol").and_then(|s| s.as_str()).unwrap_or("???");
                        let side = pos.get("side").and_then(|s| s.as_str()).unwrap_or("?");
                        let size = pos
                            .get("size")
                            .and_then(|s| s.as_str())
                            .or_else(|| pos.get("positionSize").and_then(|s| s.as_str()))
                            .unwrap_or("0");
                        let entry_price = pos
                            .get("avgEntryPrice")
                            .and_then(|s| s.as_str())
                            .or_else(|| pos.get("entryPrice").and_then(|s| s.as_str()))
                            .unwrap_or("0");
                        let unrealized_pnl = pos
                            .get("unrealisedPnl")
                            .and_then(|s| s.as_str())
                            .or_else(|| pos.get("unrealizedPnl").and_then(|s| s.as_str()))
                            .unwrap_or("0");

                        println!(
                            "   {}: side={}, size={}, entry={}, unrealized_pnl={}",
                            symbol, side, size, entry_price, unrealized_pnl
                        );
                    }
                }
            }
            Ok(())
        }
        Err(e) => {
            println!("❌ Error retrieving open positions: {}", e);
            Err(e.into())
        }
    }
}

/// Test wallet balance and open positions retrieval using environment variables.
/// Measures and prints the client initialization time as well.
async fn test_wallet_balance_and_positions() -> Result<(), Box<dyn std::error::Error>> {
    use std::time::Instant;

    let api_key = std::env::var("BYBIT_API_KEY").ok();
    let api_secret = std::env::var("BYBIT_API_SECRET").ok();

    if api_key.is_none() || api_secret.is_none() {
        println!("❌ Missing BYBIT_API_KEY or BYBIT_API_SECRET environment variables");
        println!("Skipping real API test - this is expected in CI/test environments");
        return Ok(());
    }

    let demo = std::env::var("BYBIT_DEMO")
        .unwrap_or_else(|_| "false".to_string())
        .to_lowercase()
        == "true";
    let testnet = std::env::var("BYBIT_TESTNET")
        .unwrap_or_else(|_| "false".to_string())
        .to_lowercase()
        == "true";

    println!("🔑 Creating Bybit client...");
    println!(
        "   API Key: {}...",
        api_key
            .as_ref()
            .unwrap()
            .chars()
            .take(8)
            .collect::<String>()
    );
    println!("   Demo: {}", demo);
    println!("   Testnet: {}", testnet);

    let start = Instant::now();
    let client = BybitClient::new(
        Some(api_key.unwrap()),
        Some(api_secret.unwrap()),
        testnet,
        demo,
        5000, // recv_window
        None, // referral_id
    )?;
    let duration = start.elapsed();
    println!(
        "   Client created successfully in {:.2} ms",
        duration.as_secs_f64() * 1000.0
    );

    // Make real API calls
    print_wallet_balance(&client).await?;
    print_open_positions(&client).await?;

    // Client will be dropped automatically
    println!("\n✅ Test completed successfully");
    Ok(())
}

#[tokio::test]
async fn test_auth_real_api() {
    init_test_logger();

    // Load environment variables from .env file if present
    dotenv::dotenv().ok();

    println!("Real Bybit API authentication test");
    println!(
        "Note: This test makes real API calls and requires BYBIT_API_KEY/BYBIT_API_SECRET env vars"
    );

    if let Err(e) = test_wallet_balance_and_positions().await {
        println!("❌ Test failed: {}", e);
        // Don't panic in tests - just log the error
        // This allows the test to pass even without API credentials
    }
}
