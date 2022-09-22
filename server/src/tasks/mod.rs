use crate::services::ps_move_api::PsMoveController;

pub mod effect_update_task;
pub mod ip_discovery_task;
pub mod list_controllers_task;
pub mod set_mutations_task;
pub mod update_controllers_task;

pub struct PsMoveControllers {
    pub list: Vec<Box<PsMoveController>>,
}

impl PsMoveControllers {
    pub fn new() -> PsMoveControllers {
        PsMoveControllers { list: Vec::new() }
    }
}
