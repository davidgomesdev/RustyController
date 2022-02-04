use std::{borrow::Borrow, thread::current};

use palette::Hsv;
use ps_move_api::{Breathing, LedEffect, PsMoveController, Static, Rainbow, Off};

mod ps_move_api;

fn main() {
    let api = ps_move_api::PsMoveApi::new();
    let mut controllers = api.list();

    controllers.iter_mut().for_each(|controller| {
        // controller.set_led_effect(Box::new(Static::new()), (230.0, 1.0, 0.8));
        // controller.set_led_effect(Box::new(Rainbow::new(0.1)), (321.0, 1.0, 0.8));
        // controller.set_led_effect(Box::new(Breathing::new(0.01, 0.5)), (321.0, 1.0, 1.0))
        controller.set_led_effect(Box::new(Off::new()), (230.0, 1.0, 0.8))
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
