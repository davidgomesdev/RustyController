use std::io;

use tracing::Level;
use tracing_loki::BackgroundTask;
use tracing_loki::url::Url;
use tracing_subscriber::{filter, fmt};
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use warp::hyper::Client;

const LOKI_BASE_URL: &str = "http://127.0.0.1:3100";

fn build_loki_layer() -> (tracing_loki::Layer, BackgroundTask) {
    tracing_loki::layer(
        Url::parse(LOKI_BASE_URL).unwrap(),
        vec![("host".into(), "rusty_controller".into())]
            .into_iter()
            .collect(),
        vec![].into_iter().collect(),
    )
        .unwrap()
}

pub async fn setup_tracing() {
    let file = tracing_appender::rolling::daily("logs", "daily");
    let filter = filter::Targets::new()
        .with_target("rusty_controller", Level::DEBUG)
        .with_default(Level::WARN);

    let registry = tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer().with_writer(io::stdout.and(file)));

    let http = Client::new();

    match http.get(LOKI_BASE_URL.parse().unwrap()).await {
        Ok(_) => {
            let (layer, task) = build_loki_layer();

            registry.with(layer).init();
            tokio::spawn(task);

            tracing::info!("Loki initialized");
        }
        Err(_) => {
            registry.init();

            tracing::warn!("Couldn't connect to Loki. Continuing without it.");
        }
    };
}
