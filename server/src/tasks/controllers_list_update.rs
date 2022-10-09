use std::sync::Arc;
use std::time::Duration;

use log::info;
use tokio::{task, time};
use tokio::runtime::Handle;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tokio::time::Instant;

use crate::{LedEffect, LedEffectDetails};
use crate::ps_move::api::PsMoveApi;
use crate::ps_move::controller::{MAX_LED_PWM_FREQUENCY, PsMoveController};
use crate::ps_move::models::{ConnectionType, ControllerInfo};
use crate::spawn_tasks::ShutdownSignal;

const INTERVAL_DURATION: Duration = Duration::from_millis(500);

mod on_connected_blink {
    use std::time::Duration;

    use lazy_static::lazy_static;
    use palette::Hsv;

    lazy_static! {
        pub static ref LED_COLOR: Hsv = Hsv::from_components((42.0, 1.0, 0.35));
        pub static ref DURATION: Duration = Duration::from_secs(1);
        pub static ref INTERVAL: Duration = Duration::from_millis(500);
    }
}

pub fn spawn(
    controllers: Arc<Mutex<Vec<Box<PsMoveController>>>>,
    mut api: PsMoveApi,
    mut shutdown_signal: ShutdownSignal,
) -> JoinHandle<()> {
    task::spawn_blocking(move || {
        let rt = Handle::current();
        let mut interval = time::interval(INTERVAL_DURATION);

        while !shutdown_signal.check_is_shutting_down() {
            rt.block_on(async {
                interval.tick().await;
            });

            api.refresh();

            let list_result = {
                let controllers = rt.block_on(async { controllers.lock().await });
                api.list(&controllers)
            };

            let new_controllers = api.connect_controllers(list_result.connected);

            let mut controllers = rt.block_on(async { controllers.lock().await });

            update_changed_controllers(&mut controllers, &list_result.disconnected);
            remove_disconnected_controllers(&mut controllers, &list_result.disconnected);

            new_controllers
                .into_iter()
                .for_each(|mut controller| {
                    controller.set_led_effect(LedEffect::new_expiring(LedEffectDetails::Blink {
                        hsv: *on_connected_blink::LED_COLOR,
                        last_blink: Instant::now(),
                        interval: *on_connected_blink::INTERVAL,
                    }, *on_connected_blink::DURATION));
                    add_connected_controllers(&mut controllers, controller);
                });
        }
    })
}

/// Updates controllers that were connected via both Bluetooth and USB,
/// but are now via only USB or Bluetooth.
fn update_changed_controllers(
    current_controllers: &mut Vec<Box<PsMoveController>>,
    disconnected_controllers: &Vec<ControllerInfo>,
) {
    current_controllers
        .into_iter()
        .filter(|controller| controller.connection_type == ConnectionType::USBAndBluetooth)
        .for_each(|controller| {
            let disconnected_info = disconnected_controllers
                .iter()
                .find(|other| controller.is_same_device(other));

            if disconnected_info.is_some() {
                let disconnected_info = disconnected_info.unwrap();
                let connection_type = if disconnected_info.bt_path.is_empty() {
                    ConnectionType::Bluetooth
                } else {
                    ConnectionType::USB
                };

                info!(
                    "Controller connection changed. ('{}' to {})",
                    controller.bt_address, controller.connection_type
                );
                controller.connection_type = connection_type;
            }
        });
}

fn remove_disconnected_controllers(
    current_controllers: &mut Vec<Box<PsMoveController>>,
    disconnected_controllers: &Vec<ControllerInfo>,
) {
    current_controllers.retain(|controller| {
        let is_disconnected = disconnected_controllers
            .iter()
            .any(|other| controller.is_same_device(other));

        if is_disconnected {
            info!(
                "Controller disconnected. ('{}' by {})",
                controller.bt_address, controller.connection_type
            );
        }

        !is_disconnected
    });
}

fn add_connected_controllers(
    controllers: &mut Vec<Box<PsMoveController>>,
    controller: Box<PsMoveController>,
) {
    let current_controller = controllers
        .iter_mut()
        .find(|current_controller| current_controller.bt_address == controller.bt_address);

    match current_controller {
        Some(current_controller) => {
            if controller.connection_type != current_controller.connection_type {
                current_controller.merge_with(&controller);
                info!(
                    "Controller connection changed. ('{}' to {})",
                    current_controller.bt_address, current_controller.connection_type
                );
            }
        }
        None => {
            info!(
                "New controller! ('{}' by {})",
                controller.bt_address, controller.connection_type
            );

            controller.set_led_pwm_frequency(MAX_LED_PWM_FREQUENCY);
            controllers.push(controller);
        }
    }
}
