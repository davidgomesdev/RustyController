use std::sync::Arc;

use log::info;
use tokio::sync::{Mutex, watch};

use graphql::graphql_api;
use ps_move::effects::LedEffectDetails;

use crate::logger::setup_logger;
use crate::ps_move::controller::PsMoveController;
use crate::ps_move::effects::LedEffect;
use crate::tasks::models::*;

mod graphql;
mod logger;
mod ps_move;
mod spawn_tasks;
mod tasks;

#[tokio::main]
async fn main() {
    setup_logger();

    let (tx, rx) = watch::channel(EffectChange {
        target: EffectTarget::All,
        effect: EffectChangeType::Led {
            effect: LedEffect::off(),
        },
    });
    let controllers = Arc::new(Mutex::new(Vec::<Box<PsMoveController>>::new()));

    let mut shutdown_command = spawn_tasks::run_move(rx, &controllers).await;
    graphql_api::start(Arc::new(tx), controllers).await;

    info!("Shutting down...");
    shutdown_command.shutdown().await
}
