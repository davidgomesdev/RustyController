use std::{
    borrow::Borrow,
    sync::{Arc, Mutex},
    time::Duration,
};
use std::borrow::BorrowMut;
use std::thread::current;

use juniper::futures::StreamExt;
use log::{debug, error, info};
use palette::{encoding::Srgb, Hsv};
use tokio::{sync::watch::Receiver, task::JoinError};
use tokio::task::JoinHandle;

use ps_move_api::LedEffect;

use crate::ps_move_api::MAX_LED_PWM_FREQUENCY;

use super::ps_move_api::{self, PsMoveApi, PsMoveController};

fn move_list_task(controllers: Arc<Mutex<PsMoveControllers>>, mut api: PsMoveApi) {
    tokio::spawn(async move {
        loop {
            {
                let mut updated_controllers = api.list();
                let mut controllers = controllers.lock().unwrap();

                updated_controllers.iter_mut().for_each(|controller| {
                    info!("Controller address: {}", controller.bt_address);
                    let old_controller = controllers.list.iter()
                        .find(|old_controller| {
                            return old_controller.bt_address == controller.bt_address;
                        });

                    if old_controller.is_some() {
                        let old_controller = old_controller.unwrap();
                        controller.copy_settings(old_controller);
                    }

                    controller.set_led_pwm_frequency(MAX_LED_PWM_FREQUENCY);
                });

                controllers.list = updated_controllers;
            }

            std::thread::sleep(Duration::from_millis(100));
        }
    });
}

fn set_effect_task(controllers: Arc<Mutex<PsMoveControllers>>, mut rx: Receiver<LedEffect>) {
    tokio::spawn(async move {
        while rx.changed().await.is_ok() {
            let mut controllers = controllers.lock().unwrap();
            let effect = *rx.borrow();

            info!("Received '{}' effect", effect);

            controllers.list.iter_mut().for_each(|controller| {
                debug!("Setting '{}' controller", controller.bt_address);
                controller.set_led_effect(effect);
                info!("Controller '{}' set", controller.bt_address);
            });
        }
    });
}

fn move_update_task(controllers: Arc<Mutex<PsMoveControllers>>) -> JoinHandle<()> {
    return tokio::spawn(async move {
        loop {
            {
                let mut controllers = controllers.lock().unwrap();

                controllers.list.iter_mut().for_each(|controller| {
                    let is_ok = controller.update();

                    if !is_ok {
                        error!("Error updating controller with SN '{}'!", controller.bt_address);
                    }
                });
            }
            // needed so there's some room for other tasks to own [controllers]
            std::thread::sleep(Duration::from_millis(1));
        }
    });
}

pub async fn run_move(rx: Receiver<LedEffect>) -> Result<(), JoinError> {
    let mut api = PsMoveApi::new();

    let controllers = Arc::new(Mutex::new(PsMoveControllers::new()));

    set_effect_task(Arc::clone(&controllers), rx);
    move_list_task(Arc::clone(&controllers), api);

    let update_task = move_update_task(controllers);

    return update_task.await;
}

struct PsMoveControllers {
    list: Vec<Box<PsMoveController>>,
}

impl PsMoveControllers {
    fn new() -> PsMoveControllers {
        PsMoveControllers {
            list: Vec::new()
        }
    }
}
