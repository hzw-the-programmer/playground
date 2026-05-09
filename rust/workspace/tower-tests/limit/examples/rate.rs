use std::{
    convert::Infallible,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

use tower::{Service, ServiceBuilder, ServiceExt, limit::RateLimitLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rate=debug,tower=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let mut svc = ServiceBuilder::new()
        .layer(RateLimitLayer::new(2, Duration::from_secs(3)))
        .service(Svc);

    for i in 0..10 {
        let r = svc.ready().await.unwrap().call(i).await.unwrap();
        tracing::debug!("{r}");
    }
}

struct Svc;

impl Service<i32> for Svc {
    type Response = i32;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, _ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        tracing::debug!("poll_ready");
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: i32) -> Self::Future {
        tracing::debug!("call");
        Box::pin(async move { Ok(req) })
    }
}
