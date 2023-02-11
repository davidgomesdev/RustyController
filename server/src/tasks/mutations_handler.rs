use std::sync::Arc;
use std::sync::Mutex;

use log::{debug, error, info, warn};
use tokio::sync::broadcast::Receiver;
use tokio::task::JoinHandle;

use crate::{EffectChange, EffectChangeType, EffectTarget};
use crate::ps_move::controller::PsMoveController;
use crate::spawn_tasks::InitialLedState;

pub fn spawn(
    controllers: Arc<Mutex<Vec<PsMoveController>>>,
    mut rx: Receiver<EffectChange>,
    initial_state: Arc<Mutex<InitialLedState>>,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        loop {
            match rx.recv().await {
                Ok(effect_change) => {
                    let mut controllers = controllers.lock().unwrap();
                    let target = effect_change.target;
                    let effect = effect_change.effect;

                    match target {
                        EffectTarget::All => {
                            info!("Setting effect '{}' for all controllers", effect);
                            controllers.iter_mut().for_each(|controller| {
                                mutate_controller_effect(controller, effect);
                                debug!("Controller '{}' set to {}", controller.bt_address, effect);
                            });

                            if let EffectChangeType::Led { effect } = effect {
                                let mut initial_state = initial_state.lock().unwrap();
                                *initial_state = InitialLedState::from(effect.clone());
                                debug!("Set '{}' as initial effect.", effect);
                            }
                        }
                        EffectTarget::Only { bt_addresses } => {
                            debug!(
                                "Setting effect '{}' for {} controllers only",
                                effect,
                                bt_addresses.len()
                            );
                            bt_addresses.iter().for_each(|bt_address| {
                                controllers
                                    .iter_mut()
                                    .find(|controller| controller.bt_address == *bt_address)
                                    .map_or_else(
                                        || {
                                            warn!(
                                        "The effect change had a non-existing controller! ('{}')",
                                        bt_address
                                    );
                                        },
                                        |controller| {
                                            mutate_controller_effect(controller, effect);
                                            info!(
                                                "Controller '{}' set to {}",
                                                controller.bt_address, effect
                                            );
                                        },
                                    );
                            });
                        }
                    }
                }
                Err(err) => {
                    error!("Error occurred in receiving effect update. (Cause: {})", err)
                }
            };
        }
    })
}

fn mutate_controller_effect(controller: &mut PsMoveController, effect: EffectChangeType) {
    match effect {
        EffectChangeType::Led { effect } => controller.set_led_effect(effect),
        EffectChangeType::Rumble { effect } => controller.set_rumble_effect(effect),
    }
}
