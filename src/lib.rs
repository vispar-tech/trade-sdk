//! # trade-sdk
//!
//! High-performance async trading API client for Rust with intelligent session and cache management.
//!
//! ## Architecture
//!
//! The library uses a sophisticated architecture for optimal performance:
//!
//! ### Session Management
//!
//! - **Shared Session**: `SessionManager` creates a single reqwest client with high-performance connection pooling
//! - **Individual Sessions**: Clients automatically create individual clients if shared session isn't initialized
//! - **Connection Pooling**: Up to 2000 concurrent connections with smart distribution per host
//!
//! ### Client Caching
//!
//! - **TTL Cache**: `ClientCache` caches client instances with 10-minute lifetime
//! - **Lock-Free**: No blocking operations for maximum performance
//! - **Lazy Cleanup**: Expired entries removed on access, not proactively

#![allow(clippy::too_many_arguments)]
mod caches;
mod clients;
mod error;
mod http;
mod session;
mod utils;

pub use caches::{BingxClientsCache, BybitClientsCache, ClientsCache};
pub use session::SharedSessionManager;

pub use clients::bingx;
pub use clients::bybit;
