use std::sync::{Arc, Mutex};
use std::time::Duration;

use log::info;
use tokio::task::JoinHandle;
use tokio::time;

use crate::ps_move::controller::PsMoveController;

const INTERVAL_DURATION: Duration = Duration::from_millis(1);

pub fn spawn(controllers: Arc<Mutex<Vec<Box<PsMoveController>>>>) -> JoinHandle<()> {
    tokio::spawn(async move {
        let mut interval = time::interval(INTERVAL_DURATION);

        loop {
            interval.tick().await;

            let mut controllers = controllers.lock().unwrap();
            let mut failed_addresses = Vec::<String>::new();

            controllers.iter_mut().for_each(|controller| {
                let res = controller.update();

                if res.is_err() {
                    let bt_address = &controller.bt_address;

                    info!(
                        "Controller disconnected during update. ('{}' by {:?})",
                        *bt_address, controller.connection_type
                    );

                    failed_addresses.push(bt_address.clone());
                }
            });

            controllers.retain(|c| !failed_addresses.contains(&c.bt_address));
        }
    })
}
