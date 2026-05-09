use std::{
    convert::Infallible,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

use tower::{Service, ServiceBuilder, ServiceExt, timeout::TimeoutLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "timeout=debug,tower=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let svc = ServiceBuilder::new()
        .layer(TimeoutLayer::new(Duration::from_secs(2)))
        .service(Svc);

    assert!(svc.oneshot(1).await.is_err());
}

struct Svc;

impl<Req> Service<Req> for Svc {
    type Response = i32;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, _ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        tracing::debug!("poll_ready");
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _req: Req) -> Self::Future {
        tracing::debug!("call");
        Box::pin(async {
            tokio::time::sleep(Duration::from_secs(5)).await;
            Ok(123)
        })
    }
}
