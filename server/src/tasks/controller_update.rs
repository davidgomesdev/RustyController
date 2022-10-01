use std::sync::Arc;
use std::time::Duration;

use log::info;
use tokio::{task, time};
use tokio::runtime::Handle;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

use crate::ps_move::controller::PsMoveController;
use crate::tasks::spawn_tasks::ShutdownSignal;

const INTERVAL_DURATION: Duration = Duration::from_millis(1);

pub(super) fn spawn(
    controllers: Arc<Mutex<Vec<Box<PsMoveController>>>>,
    shutdown_signal: ShutdownSignal,
) -> JoinHandle<()> {
    task::spawn_blocking(move || {
        let rt = Handle::current();
        let mut interval = time::interval(INTERVAL_DURATION);

        while !shutdown_signal.is_shutting_down() {
            let mut controllers = rt.block_on(async {
                interval.tick().await;
                controllers.lock().await
            });
            let mut failed_addresses = Vec::<String>::new();

            controllers.iter_mut().for_each(|controller| {
                let res = controller.update();

                if res.is_err() {
                    let bt_address = &controller.bt_address;

                    info!(
                        "Controller disconnected during update. ('{}' by {})",
                        *bt_address, controller.connection_type
                    );

                    failed_addresses.push(bt_address.clone());
                }
            });

            controllers.retain(|c| !failed_addresses.contains(&c.bt_address));
        }
    })
}
