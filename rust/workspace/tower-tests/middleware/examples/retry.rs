use std::{
    future,
    task::{Context, Poll},
};

use tower::{
    Service, ServiceBuilder, ServiceExt,
    retry::{Policy, RetryLayer},
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "retry=debug,tower=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let svc = ServiceBuilder::new()
        .layer(RetryLayer::new(Attempts(3)))
        .service(Svc(4));

    assert!(svc.oneshot(1).await.is_err());

    let svc = ServiceBuilder::new()
        .layer(RetryLayer::new(Attempts(4)))
        .service(Svc(4));

    assert!(svc.oneshot(1).await.is_ok());
}

#[derive(Clone)]
struct Attempts(usize);

type Req = i32;
type Res = i32;

impl<E> Policy<Req, Res, E> for Attempts {
    type Future = future::Ready<()>;

    fn retry(&mut self, _req: &mut Req, res: &mut Result<Res, E>) -> Option<Self::Future> {
        tracing::debug!("retry: {}", self.0);

        match res {
            Ok(_) => None,
            Err(_) => {
                if self.0 > 0 {
                    self.0 -= 1;
                    Some(future::ready(()))
                } else {
                    None
                }
            }
        }
    }

    fn clone_request(&mut self, req: &Req) -> Option<Req> {
        tracing::debug!("clone_request: {}", self.0);

        Some(req.clone())
    }
}

#[derive(Clone)]
struct Svc(i32);

impl Service<Req> for Svc {
    type Response = i32;
    type Error = ();
    type Future = future::Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        tracing::debug!("poll_ready: {}", self.0);

        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _req: Req) -> Self::Future {
        tracing::debug!("call: {}", self.0);

        if self.0 > 0 {
            self.0 -= 1;
            future::ready(Err(()))
        } else {
            future::ready(Ok(123))
        }
    }
}
