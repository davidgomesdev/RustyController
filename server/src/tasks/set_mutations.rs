use std::sync::Arc;

use log::{debug, info, warn};
use tokio::sync::Mutex;
use tokio::sync::watch::Receiver;
use tokio::task::JoinHandle;

use crate::{EffectTarget, LedEffectChange};
use crate::ps_move::controller::PsMoveController;

pub fn spawn(
    controllers: Arc<Mutex<Vec<Box<PsMoveController>>>>,
    mut rx: Receiver<LedEffectChange>,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        while rx.changed().await.is_ok() {
            let mut controllers = controllers.lock().await;
            let effect_change = rx.borrow().clone();
            let target = effect_change.target;
            let effect = effect_change.effect;

            match target {
                EffectTarget::All => {
                    debug!("Received a '{}' effect for all controllers", effect);
                    controllers.iter_mut().for_each(|controller| {
                        controller.set_led_effect(effect);
                        info!("Controller '{}' set to {}", controller.bt_address, effect);
                    });
                }
                EffectTarget::Only { bt_addresses } => {
                    debug!("Received a '{}' effect for {} controllers", effect, bt_addresses.len());
                    bt_addresses.iter().for_each(|bt_address| {
                        controllers.iter_mut().find(|controller| {
                            controller.bt_address == *bt_address
                        }).map_or_else(|| {
                            warn!("The effect change had a non-existing controller! ('{}')", bt_address);
                        }, |controller| {
                            controller.set_led_effect(effect);
                            info!("Controller '{}' set to {}", controller.bt_address, effect);
                        });
                    });
                }
            }
        }
    })
}
