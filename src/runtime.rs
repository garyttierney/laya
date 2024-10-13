use std::{error::Error, future::Future};

use hyper::{
    body::Incoming,
    rt::{Executor, Read, Timer, Write},
    service::Service,
    Request, Response,
};
use hyper_util::rt::{TokioIo, TokioTimer};

use crate::hyper_compat::ResponseBody;

#[cfg(all(feature = "glommio", target_os = "linux"))]
pub mod glommio;

#[cfg(feature = "rt-tokio")]
pub mod tokio;

pub trait Runtime2 {
    type Io: Read + Write + Unpin + Send + Sync + 'static;

    fn executor<F>() -> impl Executor<F> + Clone
    where
        F: Future + Sync + Send + 'static,
        F::Output: Send + 'static;

    fn timer() -> impl Timer + Send + Sync + Clone + 'static;
}

async fn handle_connection<R, S>(io: R::Io, service: S)
where
    R: Runtime2,
    S: Service<Request<Incoming>, Response = Response<ResponseBody>>
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

    let connection = http.serve_connection_with_upgrades(io, service);
}

pub trait ServerRuntime {
    type Config;
    type Connection: Read + Write + Send + Sync + Unpin + 'static;
    type Executor<F: Future + Sync + Send + 'static>: Executor<F> + Clone
    where
        F::Output: Send + 'static;
    type Timer: Timer;

    fn executor<F: Future + Sync + Send + 'static>() -> Self::Executor<F>
    where
        F::Output: Send + 'static;

    fn listen<S>(service: S, _: Self::Config)
    where
        S: Service<Request<Incoming>, Response = Response<ResponseBody>>
            + Send
            + Sync
            + Clone
            + 'static,
        S::Future: Send + 'static,

        <S as Service<hyper::Request<hyper::body::Incoming>>>::Future: Send,
        S::Error: Into<Box<dyn Error + Send + Sync>>;

    fn handle_connection<S>(service: S, io: Self::Connection)
    where
        S: Service<Request<Incoming>, Response = Response<ResponseBody>> + Send + Sync + 'static,
        S::Future: Send + Sync + 'static,
        S::Error: Into<Box<dyn Error + Send + Sync>>,
    {
        let mut http = hyper_util::server::conn::auto::Builder::new(Self::executor());
        http.http1()
            .timer(TokioTimer::new())
            .http2()
            .timer(TokioTimer::new());

        let connection = http.serve_connection_with_upgrades(io, service);
    }
}
