use std::sync::Arc;
use std::time::Duration;

use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tokio::time;

use crate::ps_move::controller::PsMoveController;
use crate::spawn_tasks::InitialLedState;

const INTERVAL_DURATION: Duration = Duration::from_millis(1);

pub fn spawn(
    controllers: Arc<Mutex<Vec<Box<PsMoveController>>>>,
    initial_state: Arc<Mutex<InitialLedState>>,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        let mut interval = time::interval(INTERVAL_DURATION);

        loop {
            interval.tick().await;

            let mut controllers = controllers.lock().await;

            controllers.iter_mut().for_each(|controller| {
                controller.transform_led();
                controller.transform_rumble();
            });

            let mut initial_state = initial_state.lock().await;
            let current_hsv = initial_state.hsv.clone();
            let effect = &mut initial_state.effect;

            initial_state.hsv = effect.details.get_updated_hsv(current_hsv);
        }
    })
}
