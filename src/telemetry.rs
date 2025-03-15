use std::env;
use std::time::Duration;

use opentelemetry::KeyValue;
use opentelemetry::global::{self};
use opentelemetry::trace::TracerProvider;
use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;
use opentelemetry_aws::trace::{XrayIdGenerator, XrayPropagator};
use opentelemetry_otlp::LogExporter;
use opentelemetry_resource_detectors::{OsResourceDetector, ProcessResourceDetector};
use opentelemetry_sdk::logs::SdkLoggerProvider;
use opentelemetry_sdk::logs::log_processor_with_async_runtime::BatchLogProcessor;
use opentelemetry_sdk::resource::EnvResourceDetector;
use opentelemetry_sdk::trace::span_processor_with_async_runtime::BatchSpanProcessor;
use opentelemetry_sdk::trace::{Sampler, SdkTracerProvider};
use opentelemetry_sdk::{Resource, runtime};
use opentelemetry_semantic_conventions::resource::SERVICE_VERSION;
use tokio::runtime::Runtime;
use tracing::Level;
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

pub struct TelemetryHandle {
    rt: Runtime,
    tracing_provider: Option<SdkTracerProvider>,
    logger_provider: Option<SdkLoggerProvider>,
}

impl TelemetryHandle {
    pub fn shutdown(mut self, timeout: Duration) {
        let _ = self
            .tracing_provider
            .take()
            .and_then(|provider| provider.shutdown().ok());
        let _ = self
            .logger_provider
            .take()
            .and_then(|provider| provider.shutdown().ok());

        self.rt.shutdown_timeout(timeout);
    }
}

pub fn install_telemetry_collector(disable_otel: bool) -> TelemetryHandle {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_time()
        .enable_io()
        .build()
        .expect("unable to construct a telemetry runtime");

    let _rt = rt.enter();

    let (tracer_provider, logger_provider) = if !disable_otel {
        global::set_text_map_propagator(XrayPropagator::new());

        let exporter = opentelemetry_otlp::SpanExporter::builder()
            .with_http()
            .build()
            .unwrap();

        let tracer_provider = SdkTracerProvider::builder()
            .with_sampler(Sampler::AlwaysOn)
            .with_id_generator(XrayIdGenerator::default())
            .with_resource(resource())
            .with_span_processor(BatchSpanProcessor::builder(exporter, runtime::Tokio).build())
            .build();

        let log_exporter = LogExporter::builder()
            .with_http()
            .build()
            .expect("Failed to create log exporter");

        let logger_provider = SdkLoggerProvider::builder()
            .with_resource(resource())
            .with_log_processor(BatchLogProcessor::builder(log_exporter, runtime::Tokio).build())
            .build();

        (Some(tracer_provider), Some(logger_provider))
    } else {
        (None, None)
    };

    let formatter = std::env::var("LAYA_LOG_FORMATTER").unwrap_or("pretty".into());
    let formatting_layer = tracing_subscriber::fmt::layer();

    tracing_subscriber::registry()
        .with(match formatter.as_str() {
            "compact" => formatting_layer.compact().boxed(),
            "json" => formatting_layer.json().boxed(),
            _ => formatting_layer.pretty().boxed(),
        })
        .with(ErrorLayer::default())
        .with(
            EnvFilter::builder()
                .with_default_directive(Level::INFO.into())
                .with_env_var("LAYA_LOG")
                .from_env_lossy()
                .add_directive("opendal=debug".parse().unwrap()),
        )
        .with(
            tracer_provider
                .clone()
                .map(|provider| OpenTelemetryLayer::new(provider.tracer("tracing-otel"))),
        )
        .with(
            logger_provider
                .as_ref()
                .map(OpenTelemetryTracingBridge::new),
        )
        .init();

    TelemetryHandle { rt, tracing_provider: tracer_provider.clone(), logger_provider }
}
