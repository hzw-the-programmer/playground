//! RUST_LOG=info cargo run --example instrument_4
//! 2026-06-10T09:09:29.191641Z  INFO outer: instrument_4: 外层逻辑
//! 2026-06-10T09:09:29.191758Z  INFO outer:inner{val=123}: instrument_4: 内层逻辑，参数: 123

use tracing::{info, instrument};

#[instrument]
fn outer() {
    info!("外层逻辑");
    inner(123);
}

#[instrument]
fn inner(val: i32) {
    info!("内层逻辑，参数: {}", val);
}

fn main() {
    tracing_subscriber::fmt::init();
    outer();
}
