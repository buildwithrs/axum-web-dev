use std::task::{Context, Poll};

use axum::{extract::Request, middleware::Next, response::Response};
use futures_util::future::BoxFuture;
use tower::{Layer, Service};

pub async fn simple_logging(req: Request, next: Next) -> Response {
    println!("start loggin request: {:?}", req);
    let resp = next.run(req).await;
    println!("getting response: {:?}", resp);
    resp
}

#[derive(Clone)]
pub struct MyLayer;

impl<S> Layer<S> for MyLayer {
    type Service = MyMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        MyMiddleware { inner }
    }
}

#[derive(Clone)]
pub struct MyMiddleware<S> {
    inner: S,
}

impl<S> Service<Request> for MyMiddleware<S>
where
    S: Service<Request, Response = Response> + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    // `BoxFuture` is a type alias for `Pin<Box<dyn Future + Send + 'a>>`
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: Request) -> Self::Future {
        println!("[MyLayer] calling req: {:?}", request);
        let future = self.inner.call(request);
        Box::pin(async move {
            let response: Response = future.await?;
            println!("[MyLayer] sending resp: {:?}", response);
            Ok(response)
        })
    }
}
