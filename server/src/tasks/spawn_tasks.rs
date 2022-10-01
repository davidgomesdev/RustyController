use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use tokio::sync::{mpsc, Mutex, watch};

use crate::LedEffectChange;
use crate::ps_move::api::PsMoveApi;
use crate::ps_move::controller::PsMoveController;
use crate::tasks::{
    controller_update, controllers_list_update, effects_update, ip_discovery, mutations_handler,
};

pub async fn run_move(
    rx: watch::Receiver<LedEffectChange>,
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
pub(super) struct ShutdownSignal {
    // "unused" on purpose, since when it goes out of scope,
    // the channel is closed and that's how the `Receiver` is notified
    _channel: mpsc::Sender<()>,
    flag: Arc<AtomicBool>,
}

impl ShutdownSignal {
    fn new(send: &mpsc::Sender<()>, shutdown_flag: &Arc<AtomicBool>) -> ShutdownSignal {
        ShutdownSignal {
            _channel: send.clone(),
            flag: shutdown_flag.clone(),
        }
    }

    pub(super) fn is_shutting_down(&self) -> bool {
        self.flag.load(Ordering::Relaxed)
    }
}
