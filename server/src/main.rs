use std::sync::Arc;

use log::{error, info};
use tokio::sync::{Mutex, watch};

use graphql::graphql_api;
use ps_move::models::LedEffect;

use crate::ps_move::controller::PsMoveController;

mod graphql;
mod spawn_tasks;
mod tasks;
mod ps_move;

#[tokio::main]
async fn main() {
    env_logger::init();

    let (tx, rx) = watch::channel(LedEffect::Off);
    let controllers = Arc::new(Mutex::new(Vec::<Box<PsMoveController>>::new()));

    spawn_tasks::run_move(rx, &controllers).await;
    match graphql_api::start(Arc::new(tx), controllers).await {
        Ok(_) => {}
        Err(err) => { error!("Couldn't start GraphQL! {}", err) }
    };

    info!("Shutting down...");
}
