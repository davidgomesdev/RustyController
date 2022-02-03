use std::thread::current;

use palette::Hsv;
use ps_move_api::{PsMoveController, Static, LedEffect};

mod ps_move_api;

fn main() {
    let api = ps_move_api::PsMoveApi::new();
    
    loop {
        let controllers = api.list();
        controllers.for_each(|mut controller| {
            controller.set_effect(Box::new(Static { color: Hsv::from_components((180.0, 1.0, 0.7)) }));
        });

        controllers.for_each(|mut controller| {
            let is_ok = controller.update();

            if !is_ok {
                panic!("Error! Beep boop");
            }
        });
    }
}
