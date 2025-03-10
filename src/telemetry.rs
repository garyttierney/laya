use std::time::Duration;

use opentelemetry::global::{self, set_tracer_provider};
use opentelemetry::trace::TracerProvider;
use opentelemetry::KeyValue;
use opentelemetry_aws::trace::XrayIdGenerator;
use opentelemetry_resource_detectors::{
    HostResourceDetector, OsResourceDetector, ProcessResourceDetector,
};
use opentelemetry_sdk::propagation::TraceContextPropagator;
use opentelemetry_sdk::resource::{EnvResourceDetector, ResourceDetector};
use opentelemetry_sdk::trace::{
    BatchConfig, RandomIdGenerator, Sampler, SdkTracerProvider, Tracer,
};
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
    Resource::builder()
        .with_service_name(env!("CARGO_PKG_NAME"))
        .with_detectors(&[
            Box::new(OsResourceDetector),
            Box::new(ProcessResourceDetector),
            Box::new(EnvResourceDetector::new()),
        ])
        .with_attribute(KeyValue::new(SERVICE_VERSION, env!("CARGO_PKG_VERSION")))
        .build()
}

pub struct Telemetry {
    rt: Runtime,
    tracing_provider: opentelemetry_sdk::trace::SdkTracerProvider,
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
    let exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_http()
        .build()
        .unwrap();

    let provider = SdkTracerProvider::builder()
        .with_sampler(Sampler::AlwaysOn)
        .with_id_generator(XrayIdGenerator::default())
        .with_resource(resource())
        .with_span_processor(opentelemetry_sdk::trace::span_processor_with_async_runtime::BatchSpanProcessor::builder(exporter, runtime::Tokio).build())        .build();

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
                .compact()
                .with_thread_names(true),
        )
        .with(OpenTelemetryLayer::new(tracer))
        .init();

    Telemetry { rt, tracing_provider: provider.clone() }
}
