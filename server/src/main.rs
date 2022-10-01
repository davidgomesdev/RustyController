use std::sync::Arc;

use log::{error, info};
use strum_macros::Display;
use tokio::sync::{Mutex, watch};

use graphql::graphql_api;
use ps_move::models::LedEffect;
use tasks::spawn_tasks;

use crate::ps_move::controller::PsMoveController;
use crate::ps_move::models::RumbleEffect;

mod graphql;
mod ps_move;
mod tasks;

#[derive(Clone)]
pub enum EffectTarget {
    All,
    Only { bt_addresses: Vec<String> },
}

#[derive(Clone, Copy, Display)]
pub enum EffectChangeType {
    Led { effect: LedEffect },
    Rumble { effect: RumbleEffect },
}

#[derive(Clone)]
pub struct EffectChange {
    target: EffectTarget,
    effect: EffectChangeType,
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let (tx, rx) = watch::channel(EffectChange {
        target: EffectTarget::All,
        effect: EffectChangeType::Led { effect: LedEffect::Off },
    });
    let controllers = Arc::new(Mutex::new(Vec::<Box<PsMoveController>>::new()));

    let mut shutdown_command = spawn_tasks::run_move(rx, &controllers).await;
    match graphql_api::start(Arc::new(tx), controllers).await {
        Ok(_) => {}
        Err(err) => {
            error!("Couldn't start GraphQL! {}", err)
        }
    };

    info!("Shutting down...");
    shutdown_command.shutdown().await
}
