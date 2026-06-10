//! RUST_LOG=warn cargo run --example e8
//! 2026-06-10T08:58:33.473107Z  WARN e8: warn
//!
//! RUST_LOG=info cargo run --example e8
//! 2026-06-10T08:58:43.593199Z  WARN e8: warn
//! 2026-06-10T08:58:43.593273Z  INFO e8: 驱动执行
//!
//! RUST_LOG=debug cargo run --example e8
//! 2026-06-10T08:58:50.813317Z  WARN motor_driver{channel=0 pwm=600}: e8: warn
//! 2026-06-10T08:58:50.813402Z  INFO motor_driver{channel=0 pwm=600}: e8: 驱动执行
//! 2026-06-10T08:58:50.813431Z DEBUG motor_driver{channel=0 pwm=600}: e8: debug

use tracing::{Level, debug, info, instrument, warn};

#[instrument(level = Level::DEBUG, name = "motor_driver")]
fn motor_control(channel: u8, pwm: u16) {
    warn!("warn");
    info!("驱动执行");
    debug!("debug")
}

fn main() {
    tracing_subscriber::fmt::init();
    motor_control(0, 600);
}
