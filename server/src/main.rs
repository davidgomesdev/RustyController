use std::{borrow::Borrow, thread::current};

use palette::{Hsv, rgb::RgbStandard, encoding::Srgb};
use ps_move_api::{LedEffect, PsMoveController};

mod ps_move_api;

fn main() {
    let api = ps_move_api::PsMoveApi::new();
    let mut controllers = api.list();
    let initial_hsv = Hsv::<Srgb, f32>::from_components((270.0, 1.0, 0.1));
    
    controllers.iter_mut().for_each(|controller| {
        // let effect = LedEffect::Off;
        // let effect = LedEffect::Static { hsv: initial_hsv };
        let effect = LedEffect::Breathing { initial_hsv, step: 0.003, peak: 1.0, inhaling: true };
        // let effect = LedEffect::Rainbow { saturation: 1.0, value: 1.0, step: 0.09 };

        controller.set_led_effect(effect);
    });

    loop {
        controllers.iter_mut().for_each(|controller| {
            let is_ok = controller.update();

            if !is_ok {
                panic!("Error! Beep boop");
            }
        });
    }
}
