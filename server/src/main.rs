use std::io::Error;

use graphql::graphql_api;
use tokio::sync::watch;

mod move_task;
mod ps_move_api;
mod graphql;

#[tokio::main]
async fn main() -> Result<(), Error>  {
    tokio::spawn(move_task::run_move());
    return graphql_api::start().await;
}
