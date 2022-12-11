use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;

use log::info;
use tokio::{task, time};
use tokio::runtime::Handle;
use tokio::sync::watch::Sender;
use tokio::task::JoinHandle;

use crate::ControllerChange;
use crate::ps_move::controller::PsMoveController;
use crate::ps_move::models::ButtonState;
use crate::spawn_tasks::ShutdownSignal;

const INTERVAL_DURATION: Duration = Duration::from_millis(10);

pub fn spawn(
    controllers: Arc<Mutex<Vec<Box<PsMoveController>>>>,
    ctrl_tx: Sender<ControllerChange>,
    mut shutdown_signal: ShutdownSignal,
) -> JoinHandle<()> {
    task::spawn_blocking(move || {
        let rt = Handle::current();

        while !shutdown_signal.check_is_shutting_down() {
            let mut controllers = rt.block_on(async {
                time::sleep(INTERVAL_DURATION).await;
                controllers.lock().unwrap()
            });
            let mut failed_addresses = Vec::<String>::new();

            controllers
                .iter_mut()
                .for_each(|controller| match controller.update() {
                    Ok(_) => {
                        controller.button_state.iter().for_each(|btn| match btn.1 {
                            ButtonState::Pressed | ButtonState::Released => {
                                info!(
                                    "Controller {} button {} changed to {}",
                                    controller.bt_address, btn.0, btn.1
                                );

                                ctrl_tx
                                    .send(ControllerChange::from_button(btn.0, btn.1)).unwrap();
                            }
                            _ => {}
                        });
                    }
                    Err(_) => {
                        let bt_address = &controller.bt_address;

                        info!(
                            "Controller disconnected during update. ('{}' by {})",
                            *bt_address, controller.connection_type
                        );

                        failed_addresses.push(bt_address.clone());
                    }
                });

            controllers.retain(|c| !failed_addresses.contains(&c.bt_address));
        }
    })
}
