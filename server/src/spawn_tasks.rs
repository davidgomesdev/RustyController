use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use tokio::sync::{mpsc, Mutex, watch};
use tokio::time::Instant;

use crate::EffectChange;
use crate::ps_move::api::PsMoveApi;
use crate::ps_move::controller::PsMoveController;
use crate::tasks::{
    controller_update, controllers_list_update, effects_update, ip_discovery, mutations_handler,
};

pub async fn run_move(
    rx: watch::Receiver<EffectChange>,
    controllers: &Arc<Mutex<Vec<Box<PsMoveController>>>>,
) -> ShutdownCommand {
    let api = PsMoveApi::new();
    let shutdown_flag = Arc::new(AtomicBool::new(false));
    let (send, recv) = mpsc::channel::<()>(1);

    mutations_handler::spawn(controllers.clone(), rx);
    effects_update::spawn(controllers.clone());
    controllers_list_update::spawn(
        controllers.clone(),
        api,
        ShutdownSignal::new(&send, &shutdown_flag),
    );
    controller_update::spawn(
        controllers.clone(),
        ShutdownSignal::new(&send, &shutdown_flag),
    );
    ip_discovery::spawn();

    ShutdownCommand {
        flag: shutdown_flag,
        channel: recv,
    }
}

pub struct ShutdownCommand {
    channel: mpsc::Receiver<()>,
    flag: Arc<AtomicBool>,
}

impl ShutdownCommand {
    pub async fn shutdown(&mut self) {
        self.flag.store(true, Ordering::Relaxed);
        self.channel.recv().await;
    }
}

/// Needed for blocking tasks, to prevent a panic when shutting down
pub struct ShutdownSignal {
    // "unused" on purpose, since when it goes out of scope,
    // the channel is closed and that's how the `Receiver` is notified
    _channel: mpsc::Sender<()>,
    last_load: Instant,
    flag: Arc<AtomicBool>,
    last_flag: bool,
}

impl ShutdownSignal {
    fn new(send: &mpsc::Sender<()>, shutdown_flag: &Arc<AtomicBool>) -> ShutdownSignal {
        ShutdownSignal {
            _channel: send.clone(),
            last_load: Instant::now(),
            flag: shutdown_flag.clone(),
            last_flag: false,
        }
    }

    /// Checks if a shutdown has been signaled
    /// (it's expensive due to synchronicity, therefore only effectively checks every 100ms)
    pub fn check_is_shutting_down(&mut self) -> bool {
        if self.last_load.elapsed().as_millis() >= 100 {
            self.last_load = Instant::now();
            self.last_flag = self.flag.load(Ordering::Relaxed);
        }

        self.last_flag
    }
}
