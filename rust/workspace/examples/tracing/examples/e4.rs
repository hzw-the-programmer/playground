//! RUST_LOG=trace cargo run --example e4
//! 2026-06-10T08:36:23.275649Z DEBUG e4: 超声波测距完成 sensor_name="ultrasonic" distance=30.5
//!
//! RUST_LOG=debug cargo run --example e4
//! 2026-06-10T08:36:48.008721Z DEBUG e4: 超声波测距完成 sensor_name="ultrasonic" distance=30.5
//!
//! RUST_LOG=info cargo run --example e4

use tracing::{Level, event};

fn main() {
    tracing_subscriber::fmt::init();

    let dist = 30.5;
    let sensor = "ultrasonic";

    // 手动构造事件
    event!(
        Level::DEBUG,
        sensor_name = sensor,
        distance = dist,
        "超声波测距完成"
    );
}
