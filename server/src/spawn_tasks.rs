use std::sync::{Arc, Mutex};

use tokio::sync::watch::Receiver;

use crate::ps_move::api::PsMoveApi;
use crate::ps_move::controller::PsMoveController;
use crate::ps_move::models::LedEffect;
use crate::tasks::{
    effect_update, ip_discovery, list_controllers,
    set_mutations, update_controllers,
};

pub async fn run_move(rx: Receiver<LedEffect>) {
    let api = PsMoveApi::new();

    let controllers = Arc::new(Mutex::new(Vec::<Box<PsMoveController>>::new()));

    set_mutations::spawn(controllers.clone(), rx);
    effect_update::spawn(controllers.clone());
    list_controllers::spawn(controllers.clone(), api);
    update_controllers::spawn(controllers);
    ip_discovery::spawn();
}
