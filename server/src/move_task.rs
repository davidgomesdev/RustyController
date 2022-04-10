use std::{
    borrow::Borrow,
    sync::{Arc, Mutex},
    time::Duration,
};
use std::thread::current;

use juniper::futures::StreamExt;
use palette::{encoding::Srgb, Hsv};
use tokio::{sync::watch::Receiver, task::JoinError};
use tokio::task::JoinHandle;

use ps_move_api::LedEffect;

use crate::ps_move_api::PsMoveController;

use super::ps_move_api::{self, PsMoveApi};

fn move_list_task(controllers: Arc<Mutex<PsMoveControllers>>, mut api: PsMoveApi) {
    tokio::spawn(async move {
        loop {
            {
                let mut updated_controllers = api.list();
                let mut controllers = controllers.lock().unwrap();

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

            println!("Received {} effect", effect);

            controllers.list.iter_mut().for_each(|controller| {
                println!("Setting controller");
                controller.set_led_effect(effect);
                println!("Controller set");
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
                        eprintln!("Error updating controller with SN '{}'!", controller.serial_number);
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

    let controllers = Arc::new(Mutex::new(PsMoveControllers::new(api.list())));

    set_effect_task(Arc::clone(&controllers), rx);
    move_list_task(Arc::clone(&controllers), api);

    let update_task = move_update_task(controllers);

    return update_task.await;
}

struct PsMoveControllers {
    list: Vec<PsMoveController>,
}

impl PsMoveControllers {
    fn new(list: Vec<PsMoveController>) -> PsMoveControllers {
        PsMoveControllers {
            list
        }
    }
}
