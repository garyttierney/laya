use std::error::Error;
use std::fmt::{Debug, Display};
use std::future::Future;

use http_body_util::combinators::BoxBody;
use hyper::body::{Bytes, Incoming};
use hyper::rt::{Executor, Read, Timer, Write};
use hyper::service::Service;
use hyper::{Request, Response};
use hyper_util::rt::{TokioIo, TokioTimer};
use tower_http::classify::{NeverClassifyEos, ServerErrorsFailureClass, StatusInRangeFailureClass};
use tower_http::trace::ResponseBody;
use tracing_subscriber::fmt::format::Full;

use crate::{LayaOptions, Options};

#[cfg(all(feature = "rt-glommio", target_os = "linux"))]
pub mod glommio;

#[cfg(feature = "rt-tokio")]
pub mod tokio;

pub trait Runtime {
    type Config: Debug;
    type Io: Read + Write + Unpin + Send + Sync + 'static;

    fn executor<F>() -> impl Executor<F> + Clone
    where
        F: Future + Sync + Send + 'static,
        F::Output: Send + 'static;

    fn timer() -> impl Timer + Send + Sync + Clone + 'static;

    fn bind<S, E>(options: LayaOptions, service: S)
    where
        S: Service<
                Request<Incoming>,
                Response = Response<
                    ResponseBody<BoxBody<Bytes, E>, NeverClassifyEos<ServerErrorsFailureClass>>,
                >,
            > + Clone
            + Send
            + Sync
            + 'static,
        S::Future: Send + Sync + 'static,
        S::Error: Error + Send + Sync + 'static,
        E: Into<Box<dyn Error + Send + Sync>> + Send + Sync + Display + 'static;
}

#[tracing::instrument(skip_all, err)]
async fn handle_connection<R, S, E>(
    io: R::Io,
    service: S,
) -> Result<(), Box<dyn Error + Send + Sync>>
where
    R: Runtime,
    S: Service<
        Request<Incoming>,
        Response = Response<
            ResponseBody<BoxBody<Bytes, E>, NeverClassifyEos<ServerErrorsFailureClass>>,
        >,
    >,
    S::Future: Send + Sync + 'static,
    S::Error: Send + Sync + Error + 'static,
    E: Into<Box<dyn Error + Send + Sync>> + Sync + Display + 'static,
{
    let mut http = hyper_util::server::conn::auto::Builder::new(R::executor());
    let timer = R::timer();

    http.http1().timer(timer.clone()).http2().timer(timer);
    http.serve_connection_with_upgrades(io, service).await
}
