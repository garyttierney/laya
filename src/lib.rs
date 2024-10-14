#![allow(unused)]

mod http;
mod iiif;
pub mod runtime;

use std::path::Path;
use std::sync::Arc;

use hyper::service::service_fn;
use runtime::Runtime;
use tracing::{info, info_span};

use crate::http::handle_request;

pub fn start<R: Runtime>(config: R::Config) {
    let startup = info_span!("startup");

    let service = service_fn(|req| async move {
        let uri = req.uri();
        info!(uri=%uri, "handling http request");
        handle_request(req, ()).await
    });
    startup.in_scope(move || R::listen(service, config));
}
