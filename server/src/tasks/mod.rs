use crate::ps_move::controller::PsMoveController;

pub mod effect_update;
pub mod ip_discovery;
pub mod list_controllers;
pub mod set_mutations;
pub mod update_controllers;

pub struct PsMoveControllers {
    pub list: Vec<Box<PsMoveController>>,
}

impl PsMoveControllers {
    pub fn new() -> PsMoveControllers {
        PsMoveControllers { list: Vec::new() }
    }
}
