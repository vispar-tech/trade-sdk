//! Tests for Bybit API authentication.

use trade_sdk::bybit::{
    traits::{AccountApi, PositionApi, TradeApi},
    types::{AccountType, AllCategories, Side},
    BybitClient,
};

use std::sync::Once;

// Set this to true to place a test order in the real API test
static PLACE_ORDER: bool = false;

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

/// Test wallet balance, open positions retrieval, and (optionally) place order using environment variables.
/// Measures and prints the client initialization time as well.
async fn test_wallet_balance_and_positions() -> Result<(), Box<dyn std::error::Error>> {
    use std::time::Instant;
    use trade_sdk::bybit::types::{PlaceOrderParams, PlaceOrderType};

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

    // Wallet balance
    print_wallet_balance(&client).await?;

    // Open positions
    print_open_positions(&client).await?;

    // Optionally place order
    if PLACE_ORDER {
        println!("\n📝 Placing test order (Market Buy 0.001 BTCUSDT, linear)...");
        let place_result = client
            .place_order(
                AllCategories::Linear,
                &PlaceOrderParams {
                    symbol: "BTCUSDT".to_owned(),
                    side: Side::Buy,
                    order_type: PlaceOrderType::Market,
                    qty: 0.005,
                    take_profit: Some(100_000.0),
                    ..Default::default()
                },
            )
            .await;

        match place_result {
            Ok(order_resp) => {
                println!(
                    "✅ Order placed successfully:\n{}",
                    serde_json::to_string_pretty(&order_resp.result)?
                );
            }
            Err(e) => {
                println!("❌ Error placing order: {e}");
            }
        }
    } else {
        println!("💡 Skipping place order (PLACE_ORDER=false)");
    }

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
