use std::sync::Arc;
use std::time::Duration;

use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tokio::time;
use tokio::time::MissedTickBehavior;

use crate::ps_move::controller::PsMoveController;
use crate::spawn_tasks::InitialLedState;

pub const INTERVAL_DURATION: Duration = Duration::from_millis(1);

pub async fn run(
    controllers: Arc<Mutex<Vec<PsMoveController>>>,
    initial_state: Arc<Mutex<InitialLedState>>,
) -> JoinHandle<()> {
    let mut interval = time::interval(INTERVAL_DURATION);

    interval.set_missed_tick_behavior(MissedTickBehavior::Burst);

    loop {
        interval.tick().await;

        {
            let mut controllers = controllers.lock().await;

            controllers.iter_mut().for_each(|controller| {
                controller.transform_led();
                controller.transform_rumble();
            });
        }

        let mut initial_state = initial_state.lock().await;
        let current_hsv = initial_state.hsv;
        let effect = &mut initial_state.effect;

        initial_state.hsv = effect.details.get_updated_hsv(current_hsv);
    }
}
