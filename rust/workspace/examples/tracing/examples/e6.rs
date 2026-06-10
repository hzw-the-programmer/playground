//! RUST_LOG=debug cargo run --example e6
//! 2026-06-10T08:48:57.576211Z  WARN task_worker{task_id=10}: e6: warn
//! 2026-06-10T08:48:57.576282Z  INFO task_worker{task_id=10}: e6: 在外部传入的 Span 中执行逻辑
//! 2026-06-10T08:48:57.576309Z DEBUG task_worker{task_id=10}: e6: debug
//!
//! RUST_LOG=info cargo run --example e6
//! 2026-06-10T08:49:23.821361Z  WARN task_worker{task_id=10}: e6: warn
//! 2026-06-10T08:49:23.821430Z  INFO task_worker{task_id=10}: e6: 在外部传入的 Span 中执行逻辑
//!
//! RUST_LOG=warn cargo run --example e6
//! 2026-06-10T08:49:33.123980Z  WARN e6: warn

use tracing::{Level, debug, info, span, warn};

fn work(_s: &tracing::Span) {
    let _guard = _s.enter();
    warn!("warn");
    info!("在外部传入的 Span 中执行逻辑");
    debug!("debug");
}

fn main() {
    tracing_subscriber::fmt::init();

    // 先创建，不进入
    let s = span!(Level::INFO, "task_worker", task_id = 10);
    work(&s);
}
