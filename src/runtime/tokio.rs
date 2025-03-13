use std::error::Error;
use std::fmt::Display;
use std::future::Future;

use http_body_util::combinators::BoxBody;
use hyper::body::{Bytes, Incoming};
use hyper::service::Service;
use hyper::{Request, Response};
use hyper_util::rt::{TokioExecutor, TokioIo, TokioTimer};
use tokio::net::{TcpListener, TcpStream};
use tower_http::classify::{NeverClassifyEos, ServerErrorsFailureClass};
use tower_http::trace::ResponseBody;
use tracing::info;

use super::{handle_connection, Runtime};
use crate::LayaOptions;

pub struct TokioRuntime;

impl Runtime for TokioRuntime {
    type Config = ();
    type Io = TokioIo<TcpStream>;

    fn executor<F>() -> impl hyper::rt::Executor<F> + Clone
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        TokioExecutor::new()
    }

    fn timer() -> impl hyper::rt::Timer + Send + Sync + Clone + 'static {
        TokioTimer::new()
    }

    fn bind<S, E>(options: LayaOptions, service: S)
    where
        S: Service<
                Request<Incoming>,
                Response = Response<
                    ResponseBody<BoxBody<Bytes, E>, NeverClassifyEos<ServerErrorsFailureClass>>,
                >,
            > + Clone
            + Send
            + 'static,
        E: Into<Box<dyn Error + Send + Sync>> + Send + Sync + Display + 'static,
        S::Error: Error + Send + Sync + 'static,
        S::Future: Send + 'static,
    {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .thread_name("laya")
            .enable_all()
            .build()
            .expect("failed to create HTTP runtime");

        let result: Result<_, std::io::Error> = rt.block_on(async move {
            let listener = TcpListener::bind(options.bind_address).await?;

            info!("Listening on {:?}", options.bind_address);

            loop {
                let (stream, _addr) = listener.accept().await?;
                let io = TokioIo::new(stream);
                let service = service.clone();
                let handler = handle_connection::<TokioRuntime, _, _>(io, service);

                tokio::spawn(handler);
            }

            Ok(())
        });

        result.expect("failed to bind HTTP server")
    }
}
