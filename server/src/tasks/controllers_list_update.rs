use std::sync::Arc;
use std::time::Duration;

use palette::Hsv;
use tokio::sync::Mutex;
use tokio::time;
use tokio::time::{Instant, MissedTickBehavior};
use crate::monitoring::metrics::CONNECTED_DEVICES_METRIC;

use crate::ps_move::api::PsMoveApi;
use crate::ps_move::controller::PsMoveController;
use crate::ps_move::effects::{LedEffect, LedEffectKind};
use crate::ps_move::models::{ConnectionType, ControllerInfo};
use crate::spawn_tasks::{InitialLedState, ShutdownSignal};

const INTERVAL_DURATION: Duration = Duration::from_millis(500);

fn get_on_connected_effect() -> LedEffect {
    LedEffect::new_expiring(
        LedEffectKind::Blink {
            hsv: Hsv::from_components((42.0, 1.0, 0.35)),
            last_blink: Instant::now(),
            interval: Duration::from_millis(500),
        },
        Duration::from_secs(1),
    )
}

pub async fn run(
    controllers: Arc<Mutex<Vec<PsMoveController>>>,
    mut api: PsMoveApi,
    mut shutdown_signal: ShutdownSignal,
    initial_state: Arc<Mutex<InitialLedState>>,
) {
    let mut interval = time::interval(INTERVAL_DURATION);

    interval.set_missed_tick_behavior(MissedTickBehavior::Delay);

    tracing::info!(
        "Listing controllers with '{}' as the initial effect.",
        initial_state.lock().await.effect
    );

    while !shutdown_signal.check_is_shutting_down() {
        {
            let initial_effect = &mut initial_state.lock().await.effect;
            if initial_effect.has_expired() {
                tracing::debug!("Initial '{}' effect has expired.", initial_effect);
                *initial_effect = LedEffect::off();
            }
        }

        interval.tick().await;

        api.refresh();

        let list_result = {
            let controllers = controllers.lock().await;
            api.list(&controllers)
        };

        let new_controllers = api.connect_controllers(list_result.connected);

        let mut controllers = controllers.lock().await;

        update_changed_controllers(&mut controllers, &list_result.disconnected);
        remove_disconnected_controllers(&mut controllers, &list_result.disconnected);

        let initial_state = initial_state.lock().await;

        new_controllers.into_iter().for_each(|mut controller| {
            let initial_effect = initial_state.effect.clone();

            let effect = if initial_effect.is_off() {
                tracing::info!(
                    "Setting on connected effect on '{}'.",
                    controller.bt_address
                );
                get_on_connected_effect()
            } else {
                tracing::info!(
                    "Setting current effect on '{}'. ({initial_effect})",
                    controller.bt_address
                );
                initial_effect
            };

            controller.set_led_effect_with_hsv(effect, initial_state.hsv);
            add_connected_controllers(&mut controllers, controller);
        });

        CONNECTED_DEVICES_METRIC.set(controllers.len() as i64);
    }
}

/// Updates controllers that were connected via both Bluetooth and USB,
/// but are now via only USB or Bluetooth.
fn update_changed_controllers(
    current_controllers: &mut [PsMoveController],
    disconnected_controllers: &[ControllerInfo],
) {
    current_controllers
        .iter_mut()
        .filter(|controller| controller.connection_type == ConnectionType::UsbAndBluetooth)
        .for_each(|controller| {
            let disconnected_info = disconnected_controllers
                .iter()
                .find(|other| controller.is_same_device(other));

            if let Some(info) = disconnected_info {
                let connection_type = if info.bt_path.is_empty() {
                    ConnectionType::Bluetooth
                } else {
                    ConnectionType::Usb
                };

                tracing::info!(
                    "Controller connection changed. ('{}' to {})",
                    controller.bt_address,
                    controller.connection_type
                );
                controller.connection_type = connection_type;
            }
        });
}

fn remove_disconnected_controllers(
    current_controllers: &mut Vec<PsMoveController>,
    disconnected_controllers: &[ControllerInfo],
) {
    current_controllers.retain(|controller| {
        let is_disconnected = disconnected_controllers
            .iter()
            .any(|other| controller.is_same_device(other));

        if is_disconnected {
            tracing::info!(
                "Controller disconnected. ('{}' by {})",
                controller.bt_address,
                controller.connection_type
            );
        }

        !is_disconnected
    });
}

fn add_connected_controllers(
    controllers: &mut Vec<PsMoveController>,
    controller: PsMoveController,
) {
    let current_controller = controllers
        .iter_mut()
        .find(|current_controller| current_controller.bt_address == controller.bt_address);

    match current_controller {
        Some(current_controller) => {
            if controller.connection_type != current_controller.connection_type {
                current_controller.merge_with(&controller);
                tracing::info!(
                    "Controller connection changed. ('{}' to {})",
                    current_controller.bt_address,
                    current_controller.connection_type
                );
            }
        }
        None => {
            tracing::info!(
                "New controller! ('{}' by {})",
                controller.bt_address,
                controller.connection_type
            );

            controllers.push(controller);
        }
    }
}
