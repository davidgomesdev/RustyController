use std::{io::Error, sync::Arc};

use tokio::sync::watch;

use graphql::graphql_api;
use ps_move_api::LedEffect;

mod move_task;
mod ps_move_api;
mod graphql;

#[tokio::main]
async fn main() -> Result<(), Error> {
    pretty_env_logger::init();
    let (tx, rx) = watch::channel(LedEffect::Off);

    tokio::spawn(move_task::run_move(rx));
    return graphql_api::start(Arc::new(tx)).await;
}
