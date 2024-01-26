use tracing::{event, instrument, span, Level};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// cargo run
// RUST_LOG=warn cargo run

fn main() {
    // tracing_subscriber::fmt()
    //     .with_max_level(tracing::Level::TRACE)
    //     .init();
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                println!("no env");
                "tracing=info".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    log::debug!("this is a log line");
    tracing::debug!("this is a tracing line");

    {
        let span = span!(Level::ERROR, "ERROR_SPAN");
        let _enter = span.enter();
        event!(Level::ERROR, "ERROR");
        event!(Level::WARN, "WARN");
        event!(Level::INFO, "INFO");
        event!(Level::DEBUG, "DEBUG");
        event!(Level::TRACE, "TRACE");
    }

    {
        let span = span!(Level::WARN, "WARN_SPAN");
        let _enter = span.enter();
        event!(Level::ERROR, "ERROR");
        event!(Level::WARN, "WARN");
        event!(Level::INFO, "INFO");
        event!(Level::DEBUG, "DEBUG");
        event!(Level::TRACE, "TRACE");
    }

    parent(0);
}

#[instrument]
pub fn parent(arg: usize) {
    event!(Level::ERROR, "ERROR");
    event!(Level::WARN, "WARN");
    event!(Level::INFO, "INFO");
    event!(Level::DEBUG, "DEBUG");
    event!(Level::TRACE, "TRACE");
    child(arg + 1);
}

#[instrument]
pub fn child(arg: usize) {
    event!(Level::ERROR, "ERROR");
    event!(Level::WARN, "WARN");
    event!(Level::INFO, "INFO");
    event!(Level::DEBUG, "DEBUG");
    event!(Level::TRACE, "TRACE");
}
