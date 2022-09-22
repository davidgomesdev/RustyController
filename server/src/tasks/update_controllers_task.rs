use std::sync::{Arc, Mutex};
use std::time::Duration;

use log::warn;
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
            let mut failed_addresses = Vec::<String>::new();

            controllers.list.iter_mut().for_each(|controller| {
                let res = controller.update();

                if res.is_err() {
                    let bt_address = &controller.bt_address;
                    warn!("Error updating controller with address '{}'!", *bt_address);
                    failed_addresses.push(bt_address.clone());
                }
            });

            controllers
                .list
                .retain(|c| !failed_addresses.contains(&c.bt_address));
        }
    })
}
