use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use log::{debug, error, info};
use tokio::{sync::watch::Receiver, time};
use tokio::net::UdpSocket;
use tokio::task::JoinHandle;

use ps_move_api::LedEffect;

use crate::ps_move_api::MAX_LED_PWM_FREQUENCY;

use super::ps_move_api::{self, PsMoveApi, PsMoveController};

const LIST_INTERVAL_MS: u64 = 500;

const HANDSHAKE_BEGIN_PORT: u16 = 31337;
const HANDSHAKE_END_PORT: u16 = 31338;
const HANDSHAKE_REQUEST: &str = "HelloDearRusty";
const HANDSHAKE_RESPONSE: &str = "HeyoDearClient";

pub async fn run_move(rx: Receiver<LedEffect>) {
    let api = PsMoveApi::new();

    let controllers = Arc::new(Mutex::new(PsMoveControllers::new()));

    spawn_set_requests_task(controllers.clone(), rx);
    spawn_effect_update_task(controllers.clone());
    spawn_controller_list_task(controllers.clone(), api);
    spawn_controller_update_task(controllers);
    spawn_ip_discovery_task();
}

fn spawn_controller_list_task(
    controllers: Arc<Mutex<PsMoveControllers>>,
    mut api: PsMoveApi,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_millis(LIST_INTERVAL_MS));

        loop {
            interval.tick().await;

            let mut controllers = controllers.lock().unwrap();
            let new_controllers = api.list(&mut controllers.list);

            new_controllers
                .into_iter()
                .for_each(|controller| update_controller_list(&mut (controllers.list), controller))
        }
    })
}

#[allow(dead_code)]
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

fn update_controller_list(
    controllers: &mut Vec<Box<PsMoveController>>,
    controller: Box<PsMoveController>,
) {
    let current_controller = controllers.iter_mut().find(|current_controller| {
        return current_controller.bt_address == controller.bt_address;
    });

    match current_controller {
        Some(current_controller) => {
            if controller.connection_type != current_controller.connection_type {
                info!(
                    "Controller connection changed to {} ('{}')",
                    controller.connection_type, controller.bt_address
                );
                current_controller.connection_type = controller.connection_type;
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

fn spawn_set_requests_task(
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

fn spawn_effect_update_task(controllers: Arc<Mutex<PsMoveControllers>>) -> JoinHandle<()> {
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_millis(1));

        loop {
            interval.tick().await;

            let mut controllers = controllers.lock().unwrap();

            controllers.list.iter_mut().for_each(|controller| {
                controller.transform_led();
            });
        }
    })
}

fn spawn_controller_update_task(controllers: Arc<Mutex<PsMoveControllers>>) -> JoinHandle<()> {
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_millis(1));

        loop {
            interval.tick().await;

            let mut controllers = controllers.lock().unwrap();

            controllers.list.iter_mut().for_each(|controller| {
                let is_ok = controller.update();

                if !is_ok {
                    error!(
                        "Error updating controller with address '{}'!",
                        controller.bt_address
                    );
                }
            });
        }
    })
}

fn spawn_ip_discovery_task() -> JoinHandle<Option<()>> {
    tokio::spawn(async {
        let socket = UdpSocket::bind(format!("0.0.0.0:{HANDSHAKE_BEGIN_PORT}"))
            .await
            .expect("Failed binding");
        info!("Binding on {}", HANDSHAKE_BEGIN_PORT);

        socket.set_broadcast(true).unwrap();

        loop {
            let mut packet = [0; 16];

            let recv = socket.recv_from(&mut packet).await;

            if recv.is_err() {
                continue;
            }

            let (_, mut src) = recv.unwrap();

            let ascii_packet = String::from_utf8_lossy(&packet[..14]);

            if !ascii_packet.starts_with(HANDSHAKE_REQUEST) {
                debug!("Received a packet that's not the Rusty handshake");
                continue;
            }

            info!(
                "Received Rusty handshake begin from {}:{}!",
                src.ip(),
                src.port()
            );

            src.set_port(HANDSHAKE_END_PORT);

            info!("Sending handshake end to {}:{}", src.ip(), src.port());

            socket
                .send_to(&HANDSHAKE_RESPONSE.as_bytes(), &src)
                .await
                .expect("Failed sending response");

            info!("Handshake with {} finished", src.ip());
        }
    })
}

struct PsMoveControllers {
    list: Vec<Box<PsMoveController>>,
}

impl PsMoveControllers {
    fn new() -> PsMoveControllers {
        PsMoveControllers { list: Vec::new() }
    }
}
