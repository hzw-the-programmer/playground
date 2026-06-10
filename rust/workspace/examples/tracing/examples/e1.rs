//! cargo run --example e1
//! 2026-06-10T08:16:55.659308Z ERROR e1: 错误日志

use tracing::{debug, error, info, trace, warn};

fn main() {
    // 最简初始化：控制台输出，默认日志级别 INFO
    tracing_subscriber::fmt::init();

    error!("错误日志");
    warn!("警告日志");
    info!("普通 info 日志");
    debug!("调试日志");
    trace!("追踪日志");
}
