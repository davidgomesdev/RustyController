use std::sync::Arc;
use std::time::Duration;

use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tokio::time;

use crate::ps_move::controller::PsMoveController;

const INTERVAL_DURATION: Duration = Duration::from_millis(5);

pub fn spawn(
    controllers: Arc<Mutex<Vec<Box<PsMoveController>>>>
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
        }
    })
}
