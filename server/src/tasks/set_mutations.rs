use std::sync::{Arc, Mutex};

use log::info;
use tokio::sync::watch::Receiver;
use tokio::task::JoinHandle;

use crate::ps_move::controller::PsMoveController;
use crate::ps_move::models::LedEffect;

pub fn spawn(
    controllers: Arc<Mutex<Vec<Box<PsMoveController>>>>,
    mut rx: Receiver<LedEffect>,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        while rx.changed().await.is_ok() {
            let mut controllers = controllers.lock().unwrap();
            let effect = *rx.borrow();

            info!("Received '{:?}' effect", effect);

            controllers.iter_mut().for_each(|controller| {
                controller.set_led_effect(effect);
                info!("Controller '{}' set to {:?}", controller.bt_address, effect);
            });
        }
    })
}
