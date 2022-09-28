use std::sync::Arc;

use log::{info, warn};
use tokio::sync::Mutex;
use tokio::sync::watch::Receiver;
use tokio::task::JoinHandle;

use crate::LedEffectChange;
use crate::ps_move::controller::PsMoveController;

pub fn spawn(
    controllers: Arc<Mutex<Vec<Box<PsMoveController>>>>,
    mut rx: Receiver<LedEffectChange>,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        while rx.changed().await.is_ok() {
            let mut controllers = controllers.lock().await;
            let effect = rx.borrow().clone();

            match effect {
                LedEffectChange::All { effect } => {
                    info!("Received a '{}' effect for all controllers", effect);
                    controllers.iter_mut().for_each(|controller| {
                        controller.set_led_effect(effect);
                        info!("Controller '{}' set to {}", controller.bt_address, effect);
                    });
                }
                LedEffectChange::Single { bt_address, effect } => {
                    info!("Received a '{}' effect for one controller", effect);
                    controllers.iter_mut().find(|controller| {
                        controller.bt_address == bt_address
                    }).map_or_else(|| {
                        warn!("The effect change was for a non-existing controller! ('{}')", bt_address);
                    }, |controller| {
                        controller.set_led_effect(effect);
                        info!("Controller '{}' set to {}", controller.bt_address, effect);
                    });
                }
                LedEffectChange::Multiple {
                    bt_addresses,
                    effect,
                } => {
                    info!("Received a '{}' effect for some controller", effect);
                    bt_addresses.iter().for_each(|bt_address| {
                        controllers.iter_mut().find(|controller| {
                            controller.bt_address == *bt_address
                        }).map_or_else(|| {
                            warn!("The multi-effect change had a non-existing controller! ('{}')", bt_address);
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
