use tracing::{Level, Span, field, info, span};

fn main() {
    tracing_subscriber::fmt::init();

    let span = span!(Level::INFO, "runtime_task", task_id = field::Empty);
    let guard = span.enter();

    // 运行时追加字段
    Span::current().record("task_id", 99);
    Span::current().record("status", "running");

    info!("任务执行中");
    drop(guard);
}
