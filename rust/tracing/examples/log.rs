use log::{debug, error, info, trace, warn};

// cargo run --example log
// [2024-01-26T12:06:09Z ERROR log] error

// RUST_LOG=warn cargo run --example log
// [2024-01-26T12:08:49Z ERROR log] error
// [2024-01-26T12:08:49Z WARN  log] warn

// RUST_LOG=info ./target/debug/examples/log.exe
// [2024-01-26T12:09:34Z ERROR log] error
// [2024-01-26T12:09:34Z WARN  log] warn
// [2024-01-26T12:09:34Z INFO  log] info

fn main() {
    env_logger::init();

    error!("error");
    warn!("warn");
    info!("info");
    debug!("debug");
    trace!("trace");
}
