use std::{io::Error, sync::Arc};

use log::{error, info};
use tokio::signal;
use tokio::sync::watch;

use graphql::graphql_api;
use ps_move_api::LedEffect;

mod graphql;
mod move_task;
mod ps_move_api;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let (tx, rx) = watch::channel(LedEffect::Off);

    let task = tokio::spawn(move_task::run_move(rx));
    match graphql_api::start(Arc::new(tx)).await {
        Ok(_) => {}
        Err(err) => { error!("Couldn't start GraphQL! {}", err) }
    };

    info!("Shutting down...");
}
