use std::sync::Arc;

use tokio::sync::{broadcast, watch};
use tokio::sync::Mutex;

use graphql::graphql_api;
use ps_move::effects::LedEffectDetails;

use crate::ps_move::controller::PsMoveController;
use crate::ps_move::models::ButtonState;
use crate::tasks::models::*;
use crate::tracing_setup::setup_tracing;

mod graphql;
mod ps_move;
mod spawn_tasks;
mod tasks;
mod tracing_setup;
mod monitoring;

#[tokio::main]
async fn main() {
    setup_tracing().await;

    let (effect_tx, effect_rx) = broadcast::channel(32);
    let (ctrl_tx, ctrl_rx) = watch::channel(ControllerChange::from_button(
        &Button::Move,
        &ButtonState::Released,
    ));
    let controllers = Arc::new(Mutex::new(Vec::<PsMoveController>::new()));

    let mut shutdown_command = spawn_tasks::run_move(effect_rx, ctrl_tx, &controllers).await;
    graphql_api::start(Arc::new(effect_tx), Mutex::new(ctrl_rx), controllers).await;

    tracing::info!("Shutting down...");
    shutdown_command.shutdown().await
}
