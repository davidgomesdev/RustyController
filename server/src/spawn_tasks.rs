use std::sync::Arc;

use tokio::sync::Mutex;
use tokio::sync::watch::Receiver;

use crate::LedEffectChange;
use crate::ps_move::api::PsMoveApi;
use crate::ps_move::controller::PsMoveController;
use crate::tasks::{
    ip_discovery, set_mutations, update_controller_effects,
    update_controllers_list, update_effects,
};

pub async fn run_move(rx: Receiver<LedEffectChange>, controllers: &Arc<Mutex<Vec<Box<PsMoveController>>>>) {
    let api = PsMoveApi::new();

    set_mutations::spawn(controllers.clone(), rx);
    update_effects::spawn(controllers.clone());
    update_controllers_list::spawn(controllers.clone(), api);
    update_controller_effects::spawn(controllers.clone());
    ip_discovery::spawn();
}
