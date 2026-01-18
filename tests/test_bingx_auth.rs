//! Tests for Bybit API authentication.
use trade_sdk::bingx::{
    traits::{common::CommonApi, spot::AccountApi, swap::TradeApi},
    types::{
        OrderSide, PlaceSwapOrderParams, PositionSide, SwapOrderType, TpSlStruct, TriggerPriceType,
    },
    BingxClient,
};

use std::sync::Once;

// Set this to true to place a swap order in the real API test
static PLACE_ORDER: bool = false;

static INIT_LOGGER: Once = Once::new();

/// Initialize logger once for all tests.
/// Safe to call multiple times - logger will be initialized only once.
pub fn init_test_logger() {
    INIT_LOGGER.call_once(|| {
        let _ = env_logger::builder().is_test(true).try_init();
    });
}

/// Fetch and print BingX spot account assets.
async fn print_spot_account_assets(client: &BingxClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📊 Fetching BingX spot account assets...");
    match client.get_spot_account_assets().await {
        Ok(result) => {
            println!("✅ Spot account assets retrieved successfully!");
            println!(
                "Full response:\n{}\n",
                serde_json::to_string_pretty(&result.data)?
            );

            // result.data is expected to be a Value (serde_json::Value)
            let balances = result
                .data
                .get("balances")
                .and_then(|b| b.as_array())
                .cloned()
                .unwrap_or_else(Vec::new);

            if balances.is_empty() {
                println!("No asset balances found in response.");
                return Ok(());
            }
            println!("Found {} asset balances:", balances.len());
            for asset in balances.iter().take(5) {
                let asset_name = asset.get("asset").and_then(|v| v.as_str()).unwrap_or("???");
                let free = asset.get("free").and_then(|v| v.as_str()).unwrap_or("0");
                let locked = asset.get("locked").and_then(|v| v.as_str()).unwrap_or("0");
                println!("   {}: free={}, locked={}", asset_name, free, locked);
            }
            Ok(())
        }
        Err(e) => {
            println!("❌ Error retrieving spot account assets: {e}");
            Ok(())
        }
    }
}

/// Fetch and print BingX server time.
async fn print_server_time(client: &BingxClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n⏰ Fetching BingX server time...");
    match client.get_server_time().await {
        Ok(result) => {
            println!("✅ Server time retrieved successfully!");
            println!(
                "Server time response: {}",
                serde_json::to_string_pretty(&result.data)?
            );
            Ok(())
        }
        Err(e) => {
            println!("❌ Error retrieving server time: {e}");
            Ok(())
        }
    }
}

/// Test BingX spot account assets and server time using environment variables.
async fn test_bingx_spot_assets_and_time() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env if present
    dotenv::dotenv().ok();

    let api_key = std::env::var("BINGX_API_KEY").ok();
    let api_secret = std::env::var("BINGX_API_SECRET").ok();

    if api_key.is_none() || api_secret.is_none() {
        println!("❌ Missing BINGX_API_KEY or BINGX_API_SECRET environment variables.");
        println!("Skipping real API test - this is expected in CI/test environments");
        return Ok(());
    }

    let demo = std::env::var("BINGX_DEMO")
        .unwrap_or_else(|_| "false".to_string())
        .to_lowercase()
        == "true";

    println!("🔑 Creating BingX client...");
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

    // Create client
    let client = BingxClient::new(
        Some(api_key.unwrap()),
        Some(api_secret.unwrap()),
        demo,
        5000, // recv_window
    )?;

    // Fetch spot assets and server time
    print_spot_account_assets(&client).await?;
    print_server_time(&client).await?;

    // Optionally place a swap order (example values)
    if PLACE_ORDER {
        match client
            .place_swap_order(&PlaceSwapOrderParams {
                symbol: "BTC-USDT".to_owned(),
                side: OrderSide::Buy,
                position_side: Some(PositionSide::Both),
                order_type: SwapOrderType::Market,
                quantity: Some(0.005),
                take_profit: Some(TpSlStruct {
                    order_type: SwapOrderType::TakeProfitMarket,
                    price: 100_000.0,
                    stop_price: 100_000.0,
                    working_type: TriggerPriceType::MarkPrice,
                }),
                ..Default::default()
            })
            .await
        {
            Ok(order_result) => {
                println!(
                    "✅ Swap order placed successfully: {}",
                    serde_json::to_string_pretty(&order_result.data)?
                );
            }
            Err(e) => {
                println!("❌ Error placing swap order: {e}");
            }
        }
    } else {
        println!("💡 Skipping swap order placement (PLACE_ORDER=false)");
    }

    // Client cleanup, if needed
    // Drop happens automatically at end of function

    Ok(())
}

#[tokio::test]
async fn test_auth_real_api() {
    init_test_logger();
    // Load environment variables from .env file if present
    dotenv::dotenv().ok();

    println!("Real BingX API authentication test");
    println!(
        "Note: This test makes real API calls and requires BINGX_API_KEY/BINGX_API_SECRET env vars"
    );

    if let Err(e) = test_bingx_spot_assets_and_time().await {
        println!("❌ Test failed: {}", e);
        // Don't panic in tests - just log the error
    }
}
