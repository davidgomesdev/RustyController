use std::{borrow::Borrow, thread::current};

use palette::Hsv;
use ps_move_api::{Breathing, LedEffect, PsMoveController, Static};

mod ps_move_api;

fn main() {
    let api = ps_move_api::PsMoveApi::new();
    let mut controllers = api.list();

    controllers.iter_mut().for_each(|controller| {
        controller.set_effect(Box::new(Breathing::new(0.001, 0.5)), (230.0, 1.0, 0.3))
    });

    loop {
        controllers.iter_mut().for_each(|controller| {
            println!("updating...");

            let is_ok = controller.update();

            if !is_ok {
                panic!("Error! Beep boop");
            }
        });
    }
}
