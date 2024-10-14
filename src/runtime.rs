use std::error::Error;
use std::future::Future;

use http_body_util::combinators::BoxBody;
use hyper::body::{Bytes, Incoming};
use hyper::rt::{Executor, Read, Timer, Write};
use hyper::service::Service;
use hyper::{Request, Response};
use hyper_util::rt::{TokioIo, TokioTimer};
use tracing_subscriber::fmt::format::Full;

#[cfg(all(feature = "rt-glommio", target_os = "linux"))]
pub mod glommio;

#[cfg(feature = "rt-tokio")]
pub mod tokio;

pub trait Runtime {
    type Config;
    type Io: Read + Write + Unpin + Send + Sync + 'static;

    fn executor<F>() -> impl Executor<F> + Clone
    where
        F: Future + Sync + Send + 'static,
        F::Output: Send + 'static;

    fn timer() -> impl Timer + Send + Sync + Clone + 'static;

    fn listen<S>(service: S, _: Self::Config)
    where
        S: Service<Request<Incoming>, Response = Response<BoxBody<Bytes, std::io::Error>>>
            + Send
            + Sync
            + Clone
            + 'static,
        S::Future: Send + Sync + 'static,

        <S as Service<hyper::Request<hyper::body::Incoming>>>::Future: Send,
        S::Error: Into<Box<dyn Error + Send + Sync>>;
}

#[tracing::instrument(skip_all, err)]
async fn handle_connection<R, S>(io: R::Io, service: S) -> Result<(), Box<dyn Error + Send + Sync>>
where
    R: Runtime,
    S: Service<Request<Incoming>, Response = Response<BoxBody<Bytes, std::io::Error>>>
        + Send
        + Sync
        + Clone
        + 'static,
    S::Future: Send + Sync + 'static,
    S::Error: Into<Box<dyn Error + Send + Sync>>,
{
    let mut http = hyper_util::server::conn::auto::Builder::new(R::executor());
    let timer = R::timer();

    http.http1().timer(timer.clone()).http2().timer(timer);
    http.serve_connection_with_upgrades(io, service).await
}
