use std::pin::{Pin, pin};
use std::task::{Context, Poll, Waker};
use tower::{Layer, Service, ServiceBuilder, ServiceExt};

pub fn test() {
    let mut svc = ServiceBuilder::new()
        .layer(Layer1)
        .layer(Layer2)
        .service(Service0);
    // let mut cx = Context::from_waker(Waker::noop());
    // let fut = async { svc.ready().await };
    // let mut fut = pin!(fut);
    // fut.as_mut().poll(&mut cx);
    svc.call(1);
}

struct Future0;

impl Future for Future0 {
    type Output = Result<i32, ()>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("Future0::poll");
        Poll::Ready(Ok(1))
    }
}

struct Service0;

impl<Req> Service<Req> for Service0 {
    type Response = i32;
    type Error = ();
    type Future = Future0;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        println!("Service0::poll_ready");
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Req) -> Self::Future {
        println!("Service0::call");
        Future0
    }
}

struct Service1<T> {
    inner: T,
}

impl<T, Request> Service<Request> for Service1<T>
where
    T: Service<Request>,
{
    type Response = T::Response;
    type Error = T::Error;
    type Future = T::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        println!("Service1::poll_ready");
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        println!("Service1::call");
        self.inner.call(req)
    }
}

struct Layer1;

impl<S> Layer<S> for Layer1 {
    type Service = Service1<S>;

    fn layer(&self, service: S) -> Self::Service {
        println!("Layer1::layer");
        Service1 { inner: service }
    }
}

struct Service2<T> {
    inner: T,
}

impl<T, Request> Service<Request> for Service2<T>
where
    T: Service<Request>,
{
    type Response = T::Response;
    type Error = T::Error;
    type Future = T::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        println!("Service2::poll_ready");
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        println!("Service2::call");
        self.inner.call(req)
    }
}

struct Layer2;

impl<S> Layer<S> for Layer2 {
    type Service = Service2<S>;

    fn layer(&self, service: S) -> Self::Service {
        println!("Layer2::layer");
        Service2 { inner: service }
    }
}
