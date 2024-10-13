#![allow(unused)]

mod http;
mod iiif;
pub mod runtime;

// TODO: split this sample code into http.
mod hyper_compat {
    use std::marker::PhantomData;
    use std::net::SocketAddr;
    use std::pin::Pin;
    use std::rc::Rc;
    use std::task::{Context, Poll};
    use std::{io, slice};

    use futures_lite::{AsyncRead, AsyncWrite, Future};
    use glommio::enclose;
    use glommio::net::{TcpListener, TcpStream};
    use glommio::sync::Semaphore;
    use hyper::body::{Body as HttpBody, Bytes, Frame, Incoming};
    use hyper::service::Service;
    use hyper::{Error, Request, Response};
    use hyper_util::server::conn::auto::Builder as ServerBuilder;

    #[derive(Clone)]
    struct HyperExecutor;

    impl<F> hyper::rt::Executor<F> for HyperExecutor
    where
        F: Future + 'static,
        F::Output: 'static,
    {
        fn execute(&self, fut: F) {
            glommio::spawn_local(fut).detach();
        }
    }

    struct HyperStream(pub TcpStream);

    impl hyper::rt::Write for HyperStream {
        fn poll_write(
            mut self: Pin<&mut Self>,
            cx: &mut Context,
            buf: &[u8],
        ) -> Poll<io::Result<usize>> {
            Pin::new(&mut self.0).poll_write(cx, buf)
        }

        fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<io::Result<()>> {
            Pin::new(&mut self.0).poll_flush(cx)
        }

        fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<io::Result<()>> {
            Pin::new(&mut self.0).poll_close(cx)
        }
    }

    impl hyper::rt::Read for HyperStream {
        fn poll_read(
            mut self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            mut buf: hyper::rt::ReadBufCursor<'_>,
        ) -> Poll<io::Result<()>> {
            unsafe {
                let read_slice = {
                    let buffer = buf.as_mut();
                    buffer.as_mut_ptr().write_bytes(0, buffer.len());
                    slice::from_raw_parts_mut(buffer.as_mut_ptr() as *mut u8, buffer.len())
                };
                Pin::new(&mut self.0).poll_read(cx, read_slice).map(|n| {
                    if let Ok(n) = n {
                        buf.advance(n);
                    }
                    Ok(())
                })
            }
        }
    }

    pub struct ResponseBody {
        data: Option<Bytes>,
    }

    impl ResponseBody {
        pub fn new<I: Into<Bytes>>(body: I) -> Self {
            ResponseBody { data: Some(body.into()) }
        }
    }

    impl HttpBody for ResponseBody {
        type Data = Bytes;
        type Error = Error;
        fn poll_frame(
            self: Pin<&mut Self>,
            _: &mut Context<'_>,
        ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
            Poll::Ready(self.get_mut().data.take().map(|d| Ok(Frame::data(d))))
        }
    }

    pub(crate) async fn serve_http2<S, R, A>(
        addr: A,
        service: S,
        max_connections: usize,
    ) -> io::Result<()>
    where
        S: Service<Request<Incoming>, Response = Response<ResponseBody>, Error = R>
            + Clone
            + 'static,
        R: std::error::Error + 'static + Send + Sync,
        A: Into<SocketAddr>,
    {
        let listener = TcpListener::bind(addr.into())?;
        let conn_control = Rc::new(Semaphore::new(max_connections as _));
        loop {
            match listener.accept().await {
                Err(x) => {
                    return Err(x.into());
                }
                Ok(stream) => {
                    stream.local_addr()?;
                    let io = HyperStream(stream);

                    glommio::spawn_local(enclose! { (service, conn_control) async move {
                        let _permit = conn_control.acquire_permit(1).await;
                        if let Err(e) = ServerBuilder::new(HyperExecutor).serve_connection(io, service).await {
                            // TODO
                        }
                    }}).detach();
                }
            }
        }
    }
}

use std::path::Path;
use std::sync::Arc;

use glommio::{CpuSet, LocalExecutorPoolBuilder, PoolPlacement};
use hyper::service::service_fn;
use runtime::ServerRuntime;
use tracing::{info, info_span};

use crate::http::handle_request;

pub fn start<R: ServerRuntime>(config: R::Config) {
    let startup = info_span!("startup");

    startup.in_scope(move || R::listen(service_fn(move |req| handle_request(req, ())), config));
}
