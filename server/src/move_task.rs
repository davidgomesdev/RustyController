use std::{
    borrow::Borrow,
    sync::{Arc, Mutex},
    time::Duration,
};

use palette::{encoding::Srgb, Hsv};
use ps_move_api::LedEffect;
use tokio::{sync::watch::Receiver, task::JoinError};

use super::ps_move_api::{self, PsMoveApi};

pub async fn run_move(mut rx: Receiver<LedEffect>) -> Result<(), JoinError> {
    let api = PsMoveApi::new();
    let controllers = Arc::new(Mutex::new(api.list()));

    {
        let controllers = Arc::clone(&controllers);

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

    let update_task = tokio::spawn(async move {
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
            // seems to be needed to set the effect above..
            std::thread::sleep(Duration::from_nanos(1));
        }
    });

    return update_task.await;
}
