//! # bybit-sdk
//!
//! High-performance async Bybit API client for Rust with intelligent session and cache management.
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
pub mod api;
pub mod cache;
pub mod client;
pub mod error;
pub mod http;
pub mod session;
pub mod traits;
pub mod types;

pub use cache::BybitClientCache;
pub use client::BybitClient;
pub use session::BybitSessionManager;
