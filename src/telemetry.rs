use std::time::Duration;

use opentelemetry::global::{self, set_tracer_provider};
use opentelemetry::trace::TracerProvider;
use opentelemetry::KeyValue;
use opentelemetry_resource_detectors::{
    HostResourceDetector, OsResourceDetector, ProcessResourceDetector,
};
use opentelemetry_sdk::propagation::TraceContextPropagator;
use opentelemetry_sdk::resource::ResourceDetector;
use opentelemetry_sdk::trace::{BatchConfig, RandomIdGenerator, Sampler, Tracer};
use opentelemetry_sdk::{runtime, Resource};
use opentelemetry_semantic_conventions::resource::{SERVICE_NAME, SERVICE_VERSION};
use opentelemetry_semantic_conventions::SCHEMA_URL;
use tokio::runtime::Runtime;
use tracing::Level;
use tracing_error::ErrorLayer;
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

fn resource() -> Resource {
    let detection_timeout = Duration::from_millis(100);
    let os = OsResourceDetector.detect(detection_timeout.clone());
    let process = ProcessResourceDetector.detect(detection_timeout.clone());
    let host = HostResourceDetector::default().detect(detection_timeout.clone());
    let server = Resource::from_schema_url(
        [
            KeyValue::new(SERVICE_NAME, env!("CARGO_PKG_NAME")),
            KeyValue::new(SERVICE_VERSION, env!("CARGO_PKG_VERSION")),
        ],
        SCHEMA_URL,
    );

    server.merge(&os).merge(&process).merge(&host)
}

pub struct Telemetry {
    rt: Runtime,
    tracing_provider: opentelemetry_sdk::trace::TracerProvider,
}

impl Telemetry {
    pub fn shutdown(self, timeout: Duration) {
        let _ = self.tracing_provider.shutdown();
        self.rt.shutdown_timeout(timeout);
    }
}

pub fn install_telemetry_collector() -> Telemetry {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_time()
        .enable_io()
        .build()
        .expect("unable to construct a telemetry runtime");

    let _rt = rt.enter();
    global::set_text_map_propagator(TraceContextPropagator::new());
    // let console_layer = console_subscriber::spawn();
    let provider = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_trace_config(
            opentelemetry_sdk::trace::Config::default()
                .with_sampler(Sampler::AlwaysOn)
                .with_id_generator(RandomIdGenerator::default())
                .with_resource(resource()),
        )
        .with_batch_config(BatchConfig::default())
        .with_exporter(opentelemetry_otlp::new_exporter().http())
        .install_batch(runtime::Tokio)
        .unwrap();

    let tracer = provider.tracer("tracing-otel");

    tracing_subscriber::registry()
        .with(ErrorLayer::default())
        .with(
            EnvFilter::builder()
                .with_default_directive(Level::INFO.into())
                .with_env_var("LAYA_LOG")
                .from_env_lossy(),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .pretty()
                .with_thread_names(true),
        )
        .with(OpenTelemetryLayer::new(tracer))
        .init();

    Telemetry { rt, tracing_provider: provider.clone() }
}
