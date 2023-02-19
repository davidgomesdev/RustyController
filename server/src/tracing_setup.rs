use std::io;

use tracing::Level;
use tracing_subscriber::{filter, fmt};
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub fn setup_tracing() {
    let file = tracing_appender::rolling::daily("logs", "daily");
    let filter = filter::Targets::new()
        .with_target("rusty_controller", Level::DEBUG)
        .with_default(Level::INFO);

    tracing_subscriber::registry()
        .with(fmt::layer().with_writer(io::stdout.and(file)))
        .with(filter)
        .init();
}
