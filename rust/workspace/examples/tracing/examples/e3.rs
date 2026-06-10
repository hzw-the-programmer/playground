//! cargo run --example e3
//! 2026-06-10T08:21:50.181829Z ERROR e3: 串口通信失败 err="uart timeout"
//!
//! RUST_LOG=warn cargo run --example e3
//! 2026-06-10T08:22:22.179476Z ERROR e3: 串口通信失败 err="uart timeout"
//! 2026-06-10T08:22:22.179557Z  WARN e3: 电压偏低 voltage=11.8
//!
//! RUST_LOG=info cargo run --example e3
//! 2026-06-10T08:25:12.757458Z ERROR e3: 串口通信失败 err="uart timeout"
//! 2026-06-10T08:25:12.757583Z  WARN e3: 电压偏低 voltage=11.8
//! 2026-06-10T08:25:12.757631Z  INFO e3: 收到指令，user: 1001, cmd: robot motor cmd
//! 2026-06-10T08:25:12.757656Z  INFO e3: 执行电机指令 user_id=1001 cmd="robot motor cmd"
//!
//! RUST_LOG=debug cargo run --example e3
//! 2026-06-10T08:26:18.075747Z ERROR e3: 串口通信失败 err="uart timeout"
//! 2026-06-10T08:26:18.075852Z  WARN e3: 电压偏低 voltage=11.8
//! 2026-06-10T08:26:18.075887Z  INFO e3: 收到指令，user: 1001, cmd: robot motor cmd
//! 2026-06-10T08:26:18.075901Z  INFO e3: 执行电机指令 user_id=1001 cmd="robot motor cmd"
//! 2026-06-10T08:26:18.075919Z DEBUG e3: 电机参数 speed=120 pwm=500
//!
//! RUST_LOG=trace cargo run --example e3
//! 2026-06-10T08:27:26.777442Z ERROR e3: 串口通信失败 err="uart timeout"
//! 2026-06-10T08:27:26.777529Z  WARN e3: 电压偏低 voltage=11.8
//! 2026-06-10T08:27:26.777560Z  INFO e3: 收到指令，user: 1001, cmd: robot motor cmd
//! 2026-06-10T08:27:26.777574Z  INFO e3: 执行电机指令 user_id=1001 cmd="robot motor cmd"
//! 2026-06-10T08:27:26.777593Z DEBUG e3: 电机参数 speed=120 pwm=500
//! 2026-06-10T08:27:26.777609Z TRACE e3: user_id=1001 content="robot motor cmd"

use tracing::{debug, error, info, trace, warn};

fn main() {
    tracing_subscriber::fmt::init();

    let user_id = 1001;
    let content = "robot motor cmd";

    error!(err = "uart timeout", "串口通信失败");
    warn!(voltage = 11.8, "电压偏低");

    info!("收到指令，user: {}, cmd: {}", user_id, content);

    info!(user_id, cmd = content, "执行电机指令");

    debug!(speed = 120, pwm = 500, "电机参数");

    trace!(user_id, content);
}
