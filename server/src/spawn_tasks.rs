use std::{
    sync::{Arc, Mutex},
};

use tokio::sync::watch::Receiver;

use ps_move_api::LedEffect;

use crate::tasks::{effect_update_task, ip_discovery_task, list_controllers_task, PsMoveControllers, set_mutations_task, update_controllers_task};

use super::ps_move_api::{self, PsMoveApi};

pub async fn run_move(rx: Receiver<LedEffect>) {
    let api = PsMoveApi::new();

    let controllers = Arc::new(Mutex::new(PsMoveControllers::new()));

    set_mutations_task::spawn(controllers.clone(), rx);
    effect_update_task::spawn(controllers.clone());
    list_controllers_task::spawn(controllers.clone(), api);
    update_controllers_task::spawn(controllers);
    ip_discovery_task::spawn();
}
