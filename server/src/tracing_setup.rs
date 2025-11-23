use std::{env, io};

use tracing::Level;
use tracing_loki::url::Url;
use tracing_loki::BackgroundTask;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{filter, fmt};
use warp::hyper::Client;

fn build_loki_layer(url: &str) -> (tracing_loki::Layer, BackgroundTask) {
    tracing_loki::layer(
        Url::parse(url).unwrap(),
        vec![("host".into(), "rusty_controller".into())]
            .into_iter()
            .collect(),
        vec![].into_iter().collect(),
    )
    .unwrap()
}

pub async fn setup_loki() {
    let filter = filter::Targets::new()
        .with_target("rusty_controller", Level::TRACE)
        .with_default(Level::WARN);

    let registry = tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer().with_writer(io::stdout));

    let http = Client::new();
    let mut is_url_provided = true;
    let loki_base_url = env::var("LOKI_BASE_URL").unwrap_or_else(|_| {
        is_url_provided = false;
        "http://127.0.0.1:3100".into()
    });

    if !is_url_provided && http.get(loki_base_url.parse().unwrap()).await.is_err() {
        registry.init();

        tracing::warn!("Couldn't connect to Loki. Continuing without it.");
        return;
    }

    let (layer, task) = build_loki_layer(&loki_base_url);

    registry.with(layer).init();
    tokio::spawn(task);

    tracing::info!("Loki initialized");
}
