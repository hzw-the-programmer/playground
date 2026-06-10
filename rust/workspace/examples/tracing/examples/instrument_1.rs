//! RUST_LOG=warn cargo run --example e7
//! 2026-06-10T08:53:49.531158Z  WARN e7: warn
//!
//! RUST_LOG=info cargo run --example e7
//! 2026-06-10T08:54:27.231985Z  WARN motor_control{channel=0 pwm=600}: e7: warn
//! 2026-06-10T08:54:27.232064Z  INFO motor_control{channel=0 pwm=600}: e7: 设置电机输出
//!
//! RUST_LOG=debug cargo run --example e7
//! 2026-06-10T08:54:41.638067Z  WARN motor_control{channel=0 pwm=600}: e7: warn
//! 2026-06-10T08:54:41.638136Z  INFO motor_control{channel=0 pwm=600}: e7: 设置电机输出
//! 2026-06-10T08:54:41.638160Z DEBUG motor_control{channel=0 pwm=600}: e7: debug

use tracing::{debug, info, instrument, warn};

// 自动生成与函数同名的 Span，默认 INFO 级别，自动记录函数参数
#[instrument]
fn motor_control(channel: u8, pwm: u16) {
    warn!("warn");
    info!("设置电机输出");
    debug!("debug");
}

fn main() {
    tracing_subscriber::fmt::init();
    motor_control(0, 600);
}
