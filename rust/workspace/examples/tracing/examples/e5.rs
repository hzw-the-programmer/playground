//! RUST_LOG=info cargo run --example e5
//! 2026-06-10T08:39:50.350074Z  INFO handle_sensor{sensor_type="lidar"}: e5: 开始处理激光雷达数据
//!
//! RUST_LOG=debug cargo run --example e5
//! 2026-06-10T08:42:18.870411Z  INFO handle_sensor{sensor_type="lidar"}: e5: 开始处理激光雷达数据
//! 2026-06-10T08:42:18.870509Z DEBUG handle_sensor{sensor_type="lidar"}:parse_raw_data{data_len=256}: e5: 解析原始雷达数据

use tracing::{Level, debug, info, span};

fn handle_sensor() {
    // 创建 Span：级别 + 名称 + 附加结构化字段
    let span = span!(Level::INFO, "handle_sensor", sensor_type = "lidar");
    // 进入当前 Span，后续所有 Event 都会归属该 Span
    let _guard = span.enter();

    info!("开始处理激光雷达数据");

    // 子 Span：嵌套链路
    let parse_span = span!(Level::DEBUG, "parse_raw_data", data_len = 256);
    let _p_guard = parse_span.enter();
    debug!("解析原始雷达数据");

    // 子守卫先销毁 → 子 Span 结束
} // _guard 销毁 → 父 Span 结束

fn main() {
    tracing_subscriber::fmt::init();
    handle_sensor();
}
