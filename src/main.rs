pub mod iiif;
pub mod image;
pub mod runtime;
pub mod storage;
pub mod telemetry;

use std::net::SocketAddr;
use std::time::Duration;

use byte_unit::Byte;
use clap::Parser;
use hyper::body::Incoming;
use hyper::header::{AUTHORIZATION, COOKIE};
use hyper::{Request, Response};
use hyper_util::service::TowerToHyperService;
use iiif::http::HttpImageService;
use iiif::service::ImageService;
use kaduceus::KakaduContext;
use opendal::services::Fs;
use opentelemetry_http::HeaderExtractor;
use storage::opendal::OpenDalStorageProvider;
use tower::ServiceBuilder;
use tower_http::sensitive_headers::SetSensitiveRequestHeadersLayer;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;
use tracing::field::Empty;
use tracing::info_span;
use tracing_opentelemetry::OpenTelemetrySpanExt;
use tracing_opentelemetry_instrumentation_sdk::http::http_server::update_span_from_response;
use tracing_opentelemetry_instrumentation_sdk::http::{
    http_flavor, http_host, http_method, url_scheme, user_agent,
};

use crate::image::codec::KaduceusImageReader;

#[derive(Clone, Default, Debug, clap::ValueEnum)]
pub enum Runtime {
    #[cfg(all(feature = "rt-glommio", target_os = "linux"))]
    Glommio,
    #[cfg(feature = "rt-tokio")]
    #[default]
    Tokio,
}

#[derive(clap::Parser, Debug)]
pub struct LayaOptions {
    /// Specifies the asynchronous runtime to execute I/O and networking tasks.
    /// This controls the underlying event loop implementation that will be used.
    #[arg(long, default_value("tokio"))]
    runtime: Runtime,

    /// Disables the transmission of trace and log data to an OpenTelemetry endpoint.
    /// When enabled, logs will only be emitted to stdout and trace events will not be available.
    #[arg(long, default_missing_value("true"))]
    disable_opentelemetry: bool,

    /// Specifies the network address to which the HTTP server will bind.
    /// Format is HOST:PORT (e.g., 127.0.0.1:8080 or 0.0.0.0:80).
    #[arg(long, short, default_value("127.0.0.1:43594"))]
    bind_address: SocketAddr,

    /// Sets the URL path prefix expected on all image requests.
    /// All API endpoints will be accessible under this prefix.
    #[arg(long, default_value("/"))]
    prefix: String,

    /// Defines the default media type for encoded images when client
    /// does not specify a preference via content negotiation.
    #[arg(long, default_value("image/jpeg"))]
    default_image_format: String,

    /// Enables development mode, which may include additional logging,
    /// more verbose errors, and disabled optimizations.
    #[arg(long, default_missing_value("true"))]
    dev: bool,

    #[command(flatten)]
    tokio_options: TokioRuntimeOptions,

    #[command(flatten)]
    image_decoder_options: ImageDecoderOptions,
}

#[derive(clap::Args, Clone, Debug)]
pub struct ImageDecoderOptions {
    #[command(flatten)]
    kakadu: KakaduOptions,
}

#[derive(clap::Args, Clone, Debug)]
pub struct KakaduOptions {
    /// Specifies the number of threads used to run image decoding operations.
    /// If not specified, defaults to the number of available CPU cores.
    #[arg(long("kakadu-decoder-threads"), help_heading("Kakadu"))]
    decoder_threads: Option<usize>,

    /// Specifies the amount of memory allocated to Kakadu in units of bytes.
    /// Accepts human-readable formats such as "50 MB", "1024 MiB", or "1GB".
    ///
    /// CAUTION: Failing to specify this option may result in unbounded memory
    /// usage and potential application crashes during image decoding.
    #[arg(long("kakadu-memory-limit"), help_heading("Kakadu"))]
    memory_limit: Option<Byte>,
}
#[derive(clap::Args, Clone, Debug)]
pub struct TokioRuntimeOptions {
    /// Specifies the number of threads allocated to HTTP listener sockets.
    /// This controls the concurrency of the socket accept operations.
    #[arg(
        long("tokio-listener-threads"),
        help_heading("Tokio"),
        default_value("1")
    )]
    listener_threads: usize,

    /// Specifies the number of threads allocated to file and network I/O operations.
    /// These threads handle all asynchronous operations involving disk or network access.
    #[arg(long("tokio-io-threads"), help_heading("Tokio"), default_value("1"))]
    io_threads: usize,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let options = LayaOptions::parse();
    let telemetry = telemetry::install_telemetry_collector(options.disable_opentelemetry);

    let kdu_context = KakaduContext::default();
    let kdu_image_reader = KaduceusImageReader::new(kdu_context);

    let image_service = ImageService::new(OpenDalStorageProvider, kdu_image_reader);
    let http_service = HttpImageService::new_with_prefix(image_service, &options.prefix);
    let tower_service = ServiceBuilder::new()
        .layer(SetSensitiveRequestHeadersLayer::new([AUTHORIZATION, COOKIE]))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|req: &Request<Incoming>| {
                    let http_method = http_method(req.method());

                    let span = info_span!(
                        "http_request",
                        http.request.method = %http_method,
                        network.protocol.version = %http_flavor(req.version()),
                        server.address = http_host(req),
                        server.port = ?req.uri().port(),
                        http.client.address = Empty,
                        user_agent.original = user_agent(req),
                        http.response.status_code = Empty,
                        url.path = req.uri().path(),
                        url.query = req.uri().query(),
                        url.scheme = url_scheme(req.uri()),
                        otel.name = Empty,
                        otel.kind = ?opentelemetry::trace::SpanKind::Server,
                        otel.status_code = Empty,
                        exception.message = Empty,
                    );

                    let extractor = HeaderExtractor(req.headers());
                    let context = opentelemetry::global::get_text_map_propagator(|propagator| {
                        propagator.extract(&extractor)
                    });
                    span.set_parent(context);
                    span
                })
                .on_response(|response: &Response<_>, _: Duration, span: &tracing::Span| {
                    update_span_from_response(span, response)
                }),
        )
        .layer(TimeoutLayer::new(Duration::from_secs(10)))
        .service(http_service);

    let hyper_service = TowerToHyperService::new(tower_service);

    match options.runtime {
        #[cfg(all(feature = "rt-glommio", target_os = "linux"))]
        Runtime::Glommio => {
            todo!()
        }

        #[cfg(feature = "rt-tokio")]
        Runtime::Tokio => runtime::tokio::serve(options, hyper_service),
    }

    telemetry.shutdown(Duration::from_secs(5));

    Ok(())
}
