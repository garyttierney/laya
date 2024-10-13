use std::error::Error;

use tracing::level_filters::LevelFilter;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Layer};

pub fn install_telemetry() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let console_layer = console_subscriber::spawn();

    // build a `Subscriber` by combining layers with a
    // `tracing_subscriber::Registry`:
    tracing_subscriber::registry()
        // add the console layer to the subscriber
        .with(console_layer)
        // add other layers...
        .with(
            tracing_subscriber::fmt::layer()
                .pretty()
                .with_thread_names(true)
                .with_filter(LevelFilter::INFO),
        )
        // .with(...)
        .init();

    Ok(())
}
