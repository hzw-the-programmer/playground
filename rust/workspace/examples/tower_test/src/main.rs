mod service;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "tower_test=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    service::test();
}
