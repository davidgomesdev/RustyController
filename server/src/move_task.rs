use std::{
    borrow::Borrow,
    sync::{Arc, Mutex},
    time::Duration,
};

use palette::{encoding::Srgb, Hsv};
use ps_move_api::{LedEffect};
use tokio::{sync::watch::Receiver, task::JoinError};

use super::ps_move_api::{self, PsMoveApi};

pub async fn run_move(mut rx: Receiver<LedEffect>) -> Result<(), JoinError> {
    let api = PsMoveApi::new();
    let controllers = Arc::new(Mutex::new(api.list()));

    {
        // let _initial_hsv = Hsv::<Srgb, f32>::from_components((270.0, 1.0, 0.01));
        let controllers = Arc::clone(&controllers);

        tokio::spawn(async move {
            while rx.changed().await.is_ok() {
                let mut controllers = controllers.lock().unwrap();
                let effect = *rx.borrow();

                println!("Received effect");

                controllers.iter_mut().for_each(|controller| {
                    println!("Setting controller");
                    // let effect = LedEffect::Off;
                    // let effect = LedEffect::Static { hsv: initial_hsv };
                    // let effect = LedEffect::Breathing {
                    //     initial_hsv,
                    //     step: 0.006,
                    //     peak: 0.4,
                    //     inhaling: true,
                    // };
                    // let effect = LedEffect::Rainbow { saturation: 1.0, value: 1.0, step: 0.09 };

                    controller.set_led_effect(effect);
                });
            }
        });
    }

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
            // seems to be needed to set the effect above..
            std::thread::sleep(Duration::from_nanos(1));
        }
    })
    .await;
}
