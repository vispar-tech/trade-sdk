# trade-sdk

High-performance async trading API client for Rust supporting BingX and Bybit exchanges with intelligent session and cache management.

## Architecture

The library uses a sophisticated architecture for optimal performance:

### Session Management

- **Shared Session**: `SharedSessionManager` creates a single reqwest client with high-performance connection pooling
- **Individual Sessions**: Clients automatically create individual clients if shared session isn't initialized
- **Connection Pooling**: Up to 2000 concurrent connections with smart distribution per host

### Client Caching

- **TTL Cache**: `BingxClientsCache` and `BybitClientsCache` cache client instances with 10-minute lifetime
- **Lock-Free**: No blocking operations for maximum performance
- **Lazy Cleanup**: Expired entries removed on access, not proactively

#### Implemented methods

```text
BybitClient methods (18):
    batch_cancel_order           get_order_history           
    batch_place_order            get_position_info           
    cancel_all_orders            get_server_time             
    cancel_order                 get_wallet_balance          
    get_account_info             place_order                 
    get_closed_pnl               set_leverage                
    get_instruments_info         set_margin_mode             
    get_kline                    set_trading_stop            
    get_open_and_closed_orders   switch_position_mode        
BingxClient methods (30):
    cancel_all_spot_open_orders                 get_spot_order_history                     
    cancel_all_swap_open_orders                 get_spot_symbols_like                      
    cancel_spot_batch_orders                    get_spot_trade_details                     
    cancel_swap_batch_orders                    get_swap_contracts                         
    change_swap_margin_type                     get_swap_klines                            
    close_swap_position                         get_swap_leverage_and_available_positions  
    get_account_asset_overview                  get_swap_margin_type                       
    get_account_asset_overview                  get_swap_open_orders                       
    get_api_permissions                         get_swap_order_details                     
    get_server_time                             get_swap_order_history                     
    get_spot_account_assets                     get_swap_position_history                  
    get_spot_account_assets                     get_swap_position_mode                     
    get_spot_klines                             place_swap_order                           
    get_spot_open_orders                        set_swap_leverage                          
    get_spot_order_details                      set_swap_position_mode 
```

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
trade-sdk = "0.1.0"
```

## Quick Start

### Option 1: Shared Session (Recommended for Production)

```rust
use trade_sdk::{SharedSessionManager, bybit::BybitClient, bingx::BingxClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize shared session at startup (once per application)
    SharedSessionManager::setup(2000);

    // Create clients for different exchanges - they automatically use the shared session
    let bybit = BybitClient::new(
        Some("bybit_key".into()),
        Some("bybit_secret".into()),
        false, false, 5000, None,
    )?;
    let bingx = BingxClient::new(
        Some("bingx_key".into()),
        Some("bingx_secret".into()),
        false, 5000,
    )?;

    // Use clients for API calls
    let bybit_balance = bybit.get_wallet_balance(None, None).await?;
    let bingx_time = bingx.get_server_time().await?;

    println!("Bybit balance: {:?}", bybit_balance);
    println!("BingX time: {:?}", bingx_time);

    // Close shared session at shutdown
    SharedSessionManager::close().await;
    Ok(())
}
```

### Option 2: Individual Sessions

```rust
use trade_sdk::{bybit::BybitClient, bingx::BingxClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Bybit client with individual session
    let bybit = BybitClient::new(
        Some("your_key".into()),
        Some("your_secret".into()),
        false, false, 5000, None,
    )?;
    let balance = bybit.get_wallet_balance(None, None).await?;
    println!("Bybit balance: {:?}", balance);

    // BingX client with individual session
    let bingx = BingxClient::new(
        Some("your_key".into()),
        Some("your_secret".into()),
        false, 5000,
    )?;
    let time = bingx.get_server_time().await?;
    println!("BingX time: {:?}", time);

    Ok(())
}
```

### Option 3: Cached Clients

```rust
use trade_sdk::{BybitClientsCache, BingxClientsCache};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get cached Bybit client (creates new if doesn't exist)
    let bybit = BybitClientsCache::get_or_create(
        Some("your_key".into()),
        Some("your_secret".into()),
        false, false, 5000, None,
    )?;

    // Get cached BingX client
    let bingx = BingxClientsCache::get_or_create(
        Some("your_key".into()),
        Some("your_secret".into()),
        false, 5000,
    )?;

    // Use clients (session management is automatic)
    let balance = bybit.get_wallet_balance(None, None).await?;
    let time = bingx.get_server_time().await?;

    println!("Bybit balance: {:?}", balance);
    println!("BingX time: {:?}", time);

    Ok(())
}
```

## Session Behavior

| Scenario                              | Session Type              | When Used              |
| ------------------------------------- | ------------------------- | ---------------------- |
| `SharedSessionManager::setup()` called | Shared session            | All clients            |
| No shared session initialized         | Individual session        | Each client            |
| Cached clients                        | Depends on initialization | Cached per credentials |

## Cache Features

- **Automatic TTL**: 10 minutes default, configurable
- **Memory Safe**: Prevents client accumulation
- **High Performance**: Lock-free operations
- **Lazy Cleanup**: Expired entries removed on access, not proactively

```rust
// Configure cache lifetime for each exchange
BybitClientsCache::configure_lifetime(std::time::Duration::from_secs(1800)); // 30 minutes
BingxClientsCache::configure_lifetime(std::time::Duration::from_secs(1800)); // 30 minutes

// Manual cleanup
let bybit_removed = BybitClientsCache::cleanup_expired();
let bingx_removed = BingxClientsCache::cleanup_expired();
```

## API Methods

### Bybit Client Methods

```rust
use trade_sdk::bybit::BybitClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = BybitClient::new(
        Some("your_key".into()),
        Some("your_secret".into()),
        false, false, 5000, None,
    )?;

    // Market data
    let server_time = client.get_server_time().await?;
    let instruments = client.get_instruments_info(
        trade_sdk::bybit::AllCategories::Spot,
        None, None, None, None, None, None,
    ).await?;
    let klines = client.get_kline(
        "BTCUSDT", "1h", Some(&trade_sdk::bybit::AllCategories::Linear),
        None, None, None,
    ).await?;

    // Account
    let balance = client.get_wallet_balance(
        Some(trade_sdk::bybit::AccountType::Unified),
        Some("BTC"),
    ).await?;
    let account_info = client.get_account_info().await?;

    // Trading
    let margin_result = client.set_margin_mode(
        trade_sdk::bybit::MarginMode::IsolatedMargin
    ).await?;

    println!("Server time: {:?}", server_time);
    println!("Balance: {:?}", balance);

    Ok(())
}
```

### BingX Client Methods

```rust
use trade_sdk::bingx::BingxClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = BingxClient::new(
        Some("your_key".into()),
        Some("your_secret".into()),
        false, 5000,
    )?;

    // Server time
    let time = client.get_server_time().await?;
    println!("Server time: {:?}", time);

    Ok(())
}
```

## Requirements

- Rust stable (`1.70+`)
- `tokio`
- High-performance connection pooling for production use

## Performance Tips

1. **Use Shared Session** for applications creating many clients
2. **Enable Caching** for repeated API credential usage
3. **Configure Connection Limits** based on your throughput needs

## Dev/TODO

- Remove null params in order (serde_json) serialization
- Improve granular error handling in all API abstractions