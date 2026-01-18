//! Test modules for trade-sdk.
//!
//! These tests are equivalent to the Python tests in python-reference/tests/
//! but adapted for Rust using built-in #[test] macros.

mod benchmark;
mod get_all_methods;

mod test_bingx_auth;
mod test_bybit_auth;
mod test_client;
mod test_multiclient;
