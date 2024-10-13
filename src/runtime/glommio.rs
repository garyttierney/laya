use super::ServerRuntime;

pub struct GlommioServerRuntime;

// impl ServerRuntime for GlommioServerRuntime {
//     type Config = ();

//     type Connection = ;

//     type Executor<F: std::future::Future + Sync + Send + 'static>
//     where
//         F::Output: Send + 'static;

//     type Timer;

//     fn listen<S>(service: S, _: Self::Config)
//     where
//         S: hyper::service::Service<
//                 hyper::Request<hyper::body::Incoming>,
//                 Response = hyper::Response<crate::hyper_compat::ResponseBody>,
//             > + Send
//             + Sync
//             + Clone
//             + 'static,
//         S::Future: Send + 'static,

//         <S as hyper::service::Service<hyper::Request<hyper::body::Incoming>>>::Future: Send,
//         S::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
//     {
//         todo!()
//     }
// }
