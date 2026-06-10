//! RUST_LOG=warn cargo run --example instrument_3
//! 2026-06-10T09:03:09.995201Z  WARN instrument_3: warn
//!
//! RUST_LOG=info cargo run --example instrument_3
//! 2026-06-10T09:03:49.064976Z  WARN process_data{id=101}: instrument_3: warn
//! 2026-06-10T09:03:49.065056Z  INFO process_data{id=101}: instrument_3: 数据处理
//!
//! RUST_LOG=debug cargo run --example instrument_3
//! 2026-06-10T09:03:59.420782Z  WARN process_data{id=101}: instrument_3: warn
//! 2026-06-10T09:03:59.420840Z  INFO process_data{id=101}: instrument_3: 数据处理
//! 2026-06-10T09:03:59.420862Z DEBUG process_data{id=101}: instrument_3: debug

use tracing::{debug, info, instrument, warn};

// 跳过 raw_data 字段不输出
#[instrument(skip(raw_data))]
fn process_data(id: u32, raw_data: &[u8]) {
    warn!("warn");
    info!("数据处理");
    debug!("debug");
    let _ = raw_data;
}

fn main() {
    tracing_subscriber::fmt::init();
    let buf = [1, 2, 3, 4];
    process_data(101, &buf);
}
