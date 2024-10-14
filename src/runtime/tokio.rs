use std::error::Error;
use std::future::Future;
use std::net::SocketAddr;

use http_body_util::combinators::BoxBody;
use hyper::body::{Bytes, Incoming};
use hyper::service::Service;
use hyper::{Request, Response};
use hyper_util::rt::{TokioExecutor, TokioIo, TokioTimer};
use tokio::net::{TcpListener, TcpStream};
use tracing::{info, info_span, Instrument};

use super::{handle_connection, Runtime};

pub struct TokioRuntime;

impl Runtime for TokioRuntime {
    type Config = ();
    type Io = TokioIo<TcpStream>;

    fn executor<F>() -> impl hyper::rt::Executor<F> + Clone
    where
        F: Future + Sync + Send + 'static,
        F::Output: Send + 'static,
    {
        TokioExecutor::new()
    }

    fn timer() -> impl hyper::rt::Timer + Send + Sync + Clone + 'static {
        TokioTimer::new()
    }

    fn listen<S>(service: S, _: Self::Config)
    where
        S: Service<Request<Incoming>, Response = Response<BoxBody<Bytes, std::io::Error>>>
            + Send
            + Sync
            + Clone
            + 'static,
        S::Future: Send + Sync + 'static,
        S::Error: Into<Box<dyn Error + Send + Sync>>,
    {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .thread_name("laya-server")
            .enable_all()
            .build()
            .unwrap();

        let error: Result<_, std::io::Error> = rt.block_on(async move {
            let listener_span = info_span!("listener");
            let addr: SocketAddr = ([127, 0, 0, 1], 43594).into();
            let listener = TcpListener::bind(addr).await?;

            info!("Listening on {addr:?}");

            loop {
                let (stream, addr) = listener.accept().await?;
                let io = TokioIo::new(stream);
                let service = service.clone();
                let handler = async move {
                    handle_connection::<TokioRuntime, _>(io, service)
                        .await
                        .unwrap()
                };

                tokio::spawn(handler.instrument(info_span!("handle", addr = ?addr)));
            }

            Ok(())
        });
    }
}
