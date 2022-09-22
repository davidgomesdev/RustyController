use std::sync::{Arc, Mutex};

use log::info;
use tokio::sync::watch::Receiver;
use tokio::task::JoinHandle;

use crate::ps_move::models::LedEffect;
use crate::tasks::PsMoveControllers;

pub fn spawn(
    controllers: Arc<Mutex<PsMoveControllers>>,
    mut rx: Receiver<LedEffect>,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        while rx.changed().await.is_ok() {
            let mut controllers = controllers.lock().unwrap();
            let effect = *rx.borrow();

            info!("Received '{}' effect", effect);

            controllers.list.iter_mut().for_each(|controller| {
                controller.set_led_effect(effect);
                info!("Controller '{}' set to {}", controller.bt_address, effect);
            });
        }
    })
}
