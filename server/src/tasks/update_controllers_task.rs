use std::sync::{Arc, Mutex};
use std::time::Duration;

use log::error;
use tokio::task::JoinHandle;
use tokio::time;

use crate::tasks::PsMoveControllers;

const INTERVAL_DURATION: Duration = Duration::from_millis(1);

pub fn spawn(controllers: Arc<Mutex<PsMoveControllers>>) -> JoinHandle<()> {
    tokio::spawn(async move {
        let mut interval = time::interval(INTERVAL_DURATION);

        loop {
            interval.tick().await;

            let mut controllers = controllers.lock().unwrap();

            controllers.list.iter_mut().for_each(|controller| {
                let res = controller.update();

                if res.is_err() {
                    error!(
                        "Error updating controller with address '{}'!",
                        controller.bt_address
                    );
                }
            });
        }
    })
}
