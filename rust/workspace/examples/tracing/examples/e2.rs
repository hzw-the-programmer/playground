//! RUST_LOG=info cargo run --example e2
//! 2026-06-10T08:13:48.700037Z ERROR e2: 错误日志
//! 2026-06-10T08:13:48.700138Z  WARN e2: 警告日志
//! 2026-06-10T08:13:48.700156Z  INFO e2: 普通 info 日志
//!
//! RUST_LOG=trace cargo run --example e2
//! 2026-06-10T08:14:44.906450Z ERROR e2: 错误日志
//! 2026-06-10T08:14:44.906534Z  WARN e2: 警告日志
//! 2026-06-10T08:14:44.906549Z  INFO e2: 普通 info 日志
//! 2026-06-10T08:14:44.906560Z DEBUG e2: 调试日志
//! 2026-06-10T08:14:44.906569Z TRACE e2: 追踪日志

use tracing::{debug, error, info, trace, warn};

fn main() {
    tracing_subscriber::fmt()
        // 读取环境变量 RUST_LOG
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    error!("错误日志");
    warn!("警告日志");
    info!("普通 info 日志");
    debug!("调试日志");
    trace!("追踪日志");
}
