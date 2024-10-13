use std::{error::Error, future::Future, net::SocketAddr};

use hyper::{body::Incoming, service::Service, Request, Response};
use hyper_util::rt::{TokioExecutor, TokioIo, TokioTimer};
use tokio::net::{TcpListener, TcpStream};

use crate::hyper_compat::ResponseBody;

use super::ServerRuntime;

pub struct TokioServerRuntime;

impl ServerRuntime for TokioServerRuntime {
    type Config = ();

    type Connection = TokioIo<TcpStream>;

    type Executor<F: Future + Sync + Send + 'static> = TokioExecutor where F::Output: Send + 'static;

    type Timer = TokioTimer;

    fn executor<F: Future + Sync + Send + 'static>() -> Self::Executor<F>
    where
        F::Output: Send + 'static,
    {
        TokioExecutor::new()
    }



    fn listen<S>(service: S, _: Self::Config)
    where
        S: Service<Request<Incoming>, Response = Response<ResponseBody>>
            + Send
            + Sync
            + Clone
            + 'static,
        S::Future: Send + 'static,
        S::Error: Into<Box<dyn Error + Send + Sync>>,
    {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .thread_name("laya-server")
            .enable_all()
            .build()
            .unwrap();

        let error: Result<_, std::io::Error> = rt.block_on(async move {
            let addr: SocketAddr = ([127, 0, 0, 1], 8089).into();
            let listener = TcpListener::bind(addr).await?;

            loop {
                let (stream, addr) = listener.accept().await?;
                let io = TokioIo::new(stream);
                let service = service.clone();

                tokio::spawn(async move {
                    let mut http =
                        hyper_util::server::conn::auto::Builder::new(TokioExecutor::new());

                    http.http1()
                        .timer(TokioTimer::new())
                        .http2()
                        .timer(TokioTimer::new());

                    let connection = http.serve_connection_with_upgrades(io, service);

                    connection.await.expect("failed to handle connection")
                });
            }

            Ok(())
        });
    }
}
