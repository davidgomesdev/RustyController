use std::sync::Arc;
use std::time::Duration;

use log::info;
use tokio::{task, time};
use tokio::runtime::Handle;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tokio::time::Instant;

use crate::ps_move::controller::PsMoveController;
use crate::spawn_tasks::ShutdownSignal;

const INTERVAL_DURATION: Duration = Duration::from_millis(1);

pub fn spawn(
    controllers: Arc<Mutex<Vec<Box<PsMoveController>>>>,
    mut shutdown_signal: ShutdownSignal,
) -> JoinHandle<()> {
    task::spawn_blocking(move || {
        let rt = Handle::current();

        while !shutdown_signal.check_is_shutting_down() {
            let mut controllers = rt.block_on(async {
                time::sleep(INTERVAL_DURATION).await;
                controllers.lock().await
            });
            let mut failed_addresses = Vec::<String>::new();

            controllers.iter_mut().for_each(|controller| {
                let inst = Instant::now();
                let res = controller.update();
                let elapsed = inst.elapsed();
                println!("took {:.2?}", elapsed);

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
