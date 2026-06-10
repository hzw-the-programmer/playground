//! RUST_LOG=warn cargo run --example async_instrument_1
//!
//! RUST_LOG=info cargo run --example async_instrument_1
//! 2026-06-10T09:07:08.075538Z  INFO async_sensor_task{sensor_id=5}: async_instrument_1: 异步传感器任务启动
//! 2026-06-10T09:07:08.285446Z  INFO async_sensor_task{sensor_id=5}: async_instrument_1: 异步任务结束

use tracing::{info, instrument};

// 异步函数直接加 instrument
#[instrument]
async fn async_sensor_task(sensor_id: u8) {
    info!("异步传感器任务启动");
    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
    info!("异步任务结束");
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    async_sensor_task(5).await;
}
