use std::{
    borrow::Borrow,
    sync::{Arc, Mutex},
    time::Duration,
};

use palette::{encoding::Srgb, Hsv};
use ps_move_api::LedEffect;
use tokio::{sync::watch::Receiver, task::JoinError};
use tokio::task::JoinHandle;
use crate::ps_move_api::PsMoveController;

use super::ps_move_api::{self, PsMoveApi};

fn set_effect_task(controllers: Arc<Mutex<Vec<PsMoveController>>>, mut rx: Receiver<LedEffect>) {
    tokio::spawn(async move {
        while rx.changed().await.is_ok() {
            let mut controllers = controllers.lock().unwrap();
            let effect = *rx.borrow();

            println!("Received effect");

            controllers.iter_mut().for_each(|controller| {
                println!("Setting controller");
                controller.set_led_effect(effect);
            });
        }
    });
}

fn move_update_task(controllers: Arc<Mutex<Vec<PsMoveController>>>) -> JoinHandle<()> {
    return tokio::spawn(async move {
        loop {
            {
                let mut controllers = controllers.lock().unwrap();

                controllers.iter_mut().for_each(|controller| {
                    let is_ok = controller.update();

                    if !is_ok {
                        panic!("Error updating controller!");
                    }
                });
            }
            // seems to be needed to use the controllers elsewhere..
            std::thread::sleep(Duration::from_nanos(1));
        }
    });
}

pub async fn run_move(rx: Receiver<LedEffect>) -> Result<(), JoinError> {
    let api = PsMoveApi::new();
    let controllers = Arc::new(Mutex::new(api.list()));

    set_effect_task(Arc::clone(&controllers), rx);
    let update_task = move_update_task(controllers);

    return update_task.await;
}
