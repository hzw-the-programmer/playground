use std::{
    convert::Infallible,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

use futures_util::future::join_all;
use tower::{Service, ServiceBuilder, ServiceExt, limit::ConcurrencyLimitLayer};

#[tokio::main]
async fn main() {
    let svc = ServiceBuilder::new()
        .layer(ConcurrencyLimitLayer::new(2))
        .service(Svc(0));

    let handles: Vec<_> = (0..10)
        .map(|i| {
            let mut svc = svc.clone();
            svc.get_mut().0 = i;
            tokio::spawn(Box::pin(async move { svc.oneshot(1).await }))
        })
        .collect();

    let ret: Vec<_> = join_all(handles)
        .await
        .into_iter()
        .map(|r| r.unwrap().unwrap())
        .collect();
    assert_eq!(ret, [123; 10]);
}

#[derive(Clone)]
struct Svc(i32);

impl<Req> Service<Req> for Svc {
    type Response = i32;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        println!("poll_ready: {}", self.0);
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _req: Req) -> Self::Future {
        println!("call: {}", self.0);
        let i = self.0;
        Box::pin(async move {
            println!("poll {}", i);
            tokio::time::sleep(Duration::from_secs(3)).await;
            Ok(123)
        })
    }
}
