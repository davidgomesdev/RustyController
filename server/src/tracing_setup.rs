use std::fs::File;
use std::io;
use std::sync::Mutex;

use futures::SinkExt;
use tracing::Level;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{fmt, Layer, Registry};
use tracing_subscriber::filter::{filter_fn, FilterExt};
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub fn setup_tracing() {
    Registry::default()
        .with(
            fmt::layer().with_writer(
                io::stdout
                    .with_max_level(Level::WARN)
                    .or_else(io::stdout.with_filter(|metadata| {
                        metadata
                            .module_path()
                            .map_or(false, |path| path.starts_with("rusty_controller"))
                    }))
            )
        )
        .init();
}
