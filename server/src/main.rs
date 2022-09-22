use std::sync::Arc;

use log::{error, info};
use tokio::sync::watch;

use graphql::graphql_api;
use ps_move_api::LedEffect;

mod graphql;
mod spawn_tasks;
mod ps_move_api;
mod tasks;

#[tokio::main]
async fn main() {
    env_logger::init();

    let (tx, rx) = watch::channel(LedEffect::Off);

    tokio::spawn(spawn_tasks::run_move(rx));
    match graphql_api::start(Arc::new(tx)).await {
        Ok(_) => {}
        Err(err) => { error!("Couldn't start GraphQL! {}", err) }
    };

    info!("Shutting down...");
}
