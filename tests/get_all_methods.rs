//! List all implemented public methods on BybitClient and BingxClient (using linkme).
//! Compact, pretty output: lists of methods in nice columns.

use trade_sdk::bingx::BINGX_IMPLEMENTED;
use trade_sdk::bybit::BYBIT_IMPLEMENTED;

/// Print the methods in a compact table (2 columns) with a given title.
fn pretty_print_methods(
    title: &str,
    methods: &[&'static str],
) {
    println!("\n{}", title);
    if methods.is_empty() {
        println!("  (none found)");
        return;
    }
    let cols = 2;
    let rows = methods.len().div_ceil(cols);
    let maxlen = methods.iter().map(|m| m.len()).max().unwrap_or(0) + 2;
    for r in 0..rows {
        for c in 0..cols {
            let idx = r + c * rows;
            if idx < methods.len() {
                print!(" {:width$}", methods[idx], width = maxlen);
            }
        }
        println!();
    }
}

#[test]
fn print_all_implemented_methods() {
    let mut bybit_methods: Vec<_> = BYBIT_IMPLEMENTED.iter().cloned().collect();
    let mut bingx_methods: Vec<_> = BINGX_IMPLEMENTED.iter().cloned().collect();

    bybit_methods.sort();
    bingx_methods.sort();

    pretty_print_methods(
        &format!("BybitClient methods ({}):", bybit_methods.len()),
        &bybit_methods,
    );
    pretty_print_methods(
        &format!("BingxClient methods ({}):", bingx_methods.len()),
        &bingx_methods,
    );
}
