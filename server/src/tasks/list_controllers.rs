use std::sync::{Arc, Mutex};
use std::time::Duration;

use log::info;
use tokio::task::JoinHandle;
use tokio::time;

use crate::ps_move::api::PsMoveApi;
use crate::ps_move::controller::{MAX_LED_PWM_FREQUENCY, PsMoveController};
use crate::ps_move::models::ControllerInfo;

const INTERVAL_DURATION: Duration = Duration::from_millis(500);

pub fn spawn(controllers: Arc<Mutex<Vec<Box<PsMoveController>>>>, mut api: PsMoveApi) -> JoinHandle<()> {
    tokio::spawn(async move {
        let mut interval = time::interval(INTERVAL_DURATION);

        loop {
            interval.tick().await;

            api.refresh();

            let list_result = {
                let controllers = controllers.lock().unwrap();
                api.list(&controllers)
            };

            let new_controllers = api.connect_controllers(list_result.connected);

            let mut controllers = controllers.lock().unwrap();
            remove_disconnected_controllers(&mut controllers, &list_result.disconnected);

            new_controllers
                .into_iter()
                .for_each(|controller| update_controller_list(&mut controllers, controller));
        }
    })
}

fn remove_disconnected_controllers(
    current_controllers: &mut Vec<Box<PsMoveController>>,
    disconnected_controllers: &Vec<ControllerInfo>,
) {
    current_controllers.retain(|controller| {
        let is_disconnected = disconnected_controllers.iter().any(|other| {
            controller.is_same_device(other)
        });

        if is_disconnected {
            info!(
                    "Controller disconnected. ('{}' by {})",
                    controller.bt_address, controller.connection_type
                )
        }

        !is_disconnected
    });
}

fn update_controller_list(
    controllers: &mut Vec<Box<PsMoveController>>,
    controller: Box<PsMoveController>,
) {
    // TODO: implement merge of controllers
    // let current_controller = controllers
    //     .iter_mut()
    //     .find(|current_controller| current_controller.bt_address == controller.bt_address);
    //
    // match current_controller {
    //     Some(current_controller) => {
    //         if controller.connection_type != current_controller.connection_type {
    //             info!(
    //                 "Controller connection changed to {} ('{}')",
    //                 controller.connection_type, controller.bt_address
    //             );
    //             current_controller.connection_type = controller.connection_type;
    //         }
    //     }
    //     None => {
    info!(
                "New controller! ('{}' by {})",
                controller.bt_address, controller.connection_type
            );
    //
    //         controller.set_led_pwm_frequency(MAX_LED_PWM_FREQUENCY);
    controllers.push(controller);
    //     }
    // }
}
