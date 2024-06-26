use std::sync::Arc;

use tokio::sync::broadcast::Receiver;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

use crate::{EffectChange, EffectChangeType, EffectTarget};
use crate::ps_move::controller::PsMoveController;
use crate::spawn_tasks::InitialLedState;

pub async fn run(
    controllers: Arc<Mutex<Vec<PsMoveController>>>,
    mut rx: Receiver<EffectChange>,
    initial_state: Arc<Mutex<InitialLedState>>,
) -> JoinHandle<()> {
    loop {
        match rx.recv().await {
            Ok(effect_change) => {
                let mut controllers = controllers.lock().await;
                let target = effect_change.target;
                let effect = effect_change.effect;

                match target {
                    EffectTarget::All => {
                        tracing::info!("Setting effect '{effect}' for all controllers");
                        controllers.iter_mut().for_each(|controller| {
                            mutate_controller_effect(controller, effect.clone());
                            tracing::debug!(
                                "Controller '{}' set to {effect}",
                                controller.bt_address
                            );
                        });

                        if let EffectChangeType::Led { effect } = effect {
                            let mut initial_state = initial_state.lock().await;
                            *initial_state = InitialLedState::from(effect.clone());
                            tracing::debug!("Set '{effect}' as initial effect.");
                        }
                    }
                    EffectTarget::Only { bt_addresses } => {
                        tracing::debug!(
                            "Setting effect '{effect}' for {} controllers only",
                            bt_addresses.len()
                        );
                        bt_addresses.iter().for_each(|bt_address| {
                            controllers
                                .iter_mut()
                                .find(|controller| controller.bt_address == *bt_address)
                                .map_or_else(
                                    || {
                                        tracing::warn!(
                                        "The effect change had a non-existing controller! ('{bt_address}')"
                                    );
                                    },
                                    |controller| {
                                        mutate_controller_effect(controller, effect.clone());
                                        tracing::info!(
                                                "Controller '{}' set to {effect}",
                                                controller.bt_address
                                            );
                                    },
                                );
                        });
                    }
                }
            }
            Err(err) => {
                tracing::error!("Error occurred in receiving effect update. (Cause: {err})")
            }
        };
    }
}

fn mutate_controller_effect(controller: &mut PsMoveController, effect: EffectChangeType) {
    match effect {
        EffectChangeType::RevertLed => { controller.revert_led_effect() }
        EffectChangeType::Led { effect } => controller.set_led_effect(effect),
        EffectChangeType::Rumble { effect } => controller.set_rumble_effect(effect),
    }
}
