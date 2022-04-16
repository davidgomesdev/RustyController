use std::{
    borrow::Borrow,
    sync::{Arc, Mutex},
    time::Duration,
};
use std::borrow::BorrowMut;
use std::sync::MutexGuard;
use std::thread::current;
use std::time::Instant;

use juniper::futures::StreamExt;
use log::{debug, error, info};
use palette::{encoding::Srgb, Hsv};
use tokio::{sync::watch::Receiver, task::JoinError, time};
use tokio::task::JoinHandle;
use tokio::time::Interval;

use ps_move_api::LedEffect;

use crate::ps_move_api::MAX_LED_PWM_FREQUENCY;

use super::ps_move_api::{self, PsMoveApi, PsMoveController};

const LIST_INTERVAL_MS: u64 = 50;

fn spawn_list_task(
    controllers: Arc<Mutex<PsMoveControllers>>,
    mut api: PsMoveApi,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_millis(LIST_INTERVAL_MS));

        loop {
            interval.tick().await;

            let mut updated_controllers = api.list();
            let mut controllers = controllers.lock().unwrap();

            controllers
                .list
                .retain(|curr| is_connected(&mut updated_controllers, curr));

            updated_controllers
                .into_iter()
                .for_each(|controller| update_list(&mut (controllers.list), controller))
        }
    })
}

fn is_connected(
    updated_controllers: &Vec<Box<PsMoveController>>,
    controller: &Box<PsMoveController>,
) -> bool {
    let updated_controller = updated_controllers
        .iter()
        .find(|ctrl| ctrl.bt_address == controller.bt_address);
    let is_connected = updated_controller.is_some();

    if !is_connected {
        info!(
            "Controller disconnected ({} by {})",
            controller.bt_address, controller.connection_type
        );
    }
    is_connected
}

fn update_list(controllers: &mut Vec<Box<PsMoveController>>, controller: Box<PsMoveController>) {
    let current_controller = controllers.iter_mut().find(|current_controller| {
        return current_controller.bt_address == controller.bt_address;
    });

    if current_controller.is_some() {
        let current_controller = current_controller.unwrap();

        if controller.connection_type != current_controller.connection_type {
            info!(
                "Controller changed ({} to {})",
                controller.bt_address, controller.connection_type
            );
            current_controller.connection_type = controller.connection_type;
        }
    } else {
        info!(
            "New controller! ({} by {})",
            controller.bt_address, controller.connection_type
        );

        controller.set_led_pwm_frequency(MAX_LED_PWM_FREQUENCY);
        controllers.push(controller);
    }
}

fn spawn_set_effect_task(
    controllers: Arc<Mutex<PsMoveControllers>>,
    mut rx: Receiver<LedEffect>,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        while rx.changed().await.is_ok() {
            let mut controllers = controllers.lock().unwrap();
            let effect = *rx.borrow();

            info!("Received '{}' effect", effect);

            controllers.list.iter_mut().for_each(|controller| {
                controller.set_led_effect(effect);
                info!("Controller '{}' set to {}", controller.bt_address, effect);
            });
        }
    })
}

fn spawn_update_task(controllers: Arc<Mutex<PsMoveControllers>>) -> JoinHandle<()> {
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_nanos(1));

        loop {
            interval.tick().await;
            let mut controllers = controllers.lock().unwrap();

            controllers.list.iter_mut().for_each(|controller| {
                let is_ok = controller.update();

                if !is_ok {
                    error!(
                        "Error updating controller with SN '{}'!",
                        controller.bt_address
                    );
                }
            })
        }
    })
}

pub async fn run_move(rx: Receiver<LedEffect>) {
    let mut api = PsMoveApi::new();

    let controllers = Arc::new(Mutex::new(PsMoveControllers::new()));

    spawn_set_effect_task(controllers.clone(), rx);
    spawn_list_task(controllers.clone(), api);
    spawn_update_task(controllers);
}

struct PsMoveControllers {
    list: Vec<Box<PsMoveController>>,
}

impl PsMoveControllers {
    fn new() -> PsMoveControllers {
        PsMoveControllers { list: Vec::new() }
    }
}
