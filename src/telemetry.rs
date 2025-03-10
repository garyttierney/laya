use std::env;
use std::time::Duration;

use opentelemetry::global::{self};
use opentelemetry::trace::TracerProvider;
use opentelemetry::KeyValue;
use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;
use opentelemetry_aws::trace::XrayIdGenerator;
use opentelemetry_otlp::LogExporter;
use opentelemetry_resource_detectors::{
    OsResourceDetector, ProcessResourceDetector,
};
use opentelemetry_sdk::logs::SdkLoggerProvider;
use opentelemetry_sdk::propagation::TraceContextPropagator;
use opentelemetry_sdk::resource::EnvResourceDetector;
use opentelemetry_sdk::trace::{Sampler, SdkTracerProvider};
use opentelemetry_sdk::{runtime, Resource};
use opentelemetry_semantic_conventions::resource::SERVICE_VERSION;
use tokio::runtime::Runtime;
use tracing::{Level, Subscriber};
use tracing_error::ErrorLayer;
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Layer};

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
    logger_provider: SdkLoggerProvider,
}

impl Telemetry {
    pub fn shutdown(self, timeout: Duration) {
        let _ = self.tracing_provider.shutdown();
        let _ = self.logger_provider.shutdown();

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

    let tracer_provider = SdkTracerProvider::builder()
        .with_sampler(Sampler::AlwaysOn)
        .with_id_generator(XrayIdGenerator::default())
        .with_resource(resource())
        .with_span_processor(opentelemetry_sdk::trace::span_processor_with_async_runtime::BatchSpanProcessor::builder(exporter, runtime::Tokio).build())
        .build();

    let log_exporter = LogExporter::builder()
        .with_http()
        .build()
        .expect("Failed to create log exporter");

    let logger_provider = SdkLoggerProvider::builder()
        .with_resource(resource())
        .with_log_processor(
            opentelemetry_sdk::logs::log_processor_with_async_runtime::BatchLogProcessor::builder(
                log_exporter,
                runtime::Tokio,
            )
            .build(),
        )
        .build();

    let tracer = tracer_provider.tracer("tracing-otel");
    let formatter = std::env::var("LAYA_LOG_FORMATTER").unwrap_or("compact".into());

    tracing_subscriber::registry()
        .with(ErrorLayer::default())
        .with(
            EnvFilter::builder()
                .with_default_directive(Level::INFO.into())
                .with_env_var("LAYA_LOG")
                .from_env_lossy(),
        )
        .with(OpenTelemetryLayer::new(tracer))
        .with(OpenTelemetryTracingBridge::new(&logger_provider))
        .with(match formatter.as_str() {
            "compact" => tracing_subscriber::fmt::layer().compact().boxed(),
            "json" => tracing_subscriber::fmt::layer().json().boxed(),
            "pretty" | _ => tracing_subscriber::fmt::layer().pretty().boxed(),
        })
        .init();

    Telemetry { rt, tracing_provider: tracer_provider.clone(), logger_provider }
}
