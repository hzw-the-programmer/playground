use tracing::info;
use tracing_subscriber::fmt::writer::MakeWriterExt;

fn main() {
    // Log all events to a rolling log file.
    let logfile = tracing_appender::rolling::hourly("./logs", "myapp-logs");
    // Log `INFO` and above to stdout.
    let stdout = std::io::stdout.with_max_level(tracing::Level::INFO);

    tracing_subscriber::fmt()
        // Combine the stdout and log file `MakeWriter`s into one
        // `MakeWriter` that writes to both
        .with_writer(stdout.and(logfile))
        .init();

    info!("机器人系统启动");
    info!("ROS2 节点初始化完成");
}
