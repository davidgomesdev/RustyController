use std::iter::Iterator;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use lazy_static::lazy_static;
use palette::Hsv;
use tokio::runtime::Handle;
use tokio::sync::{mpsc, Mutex};
use tokio::sync::broadcast::Receiver;
use tokio::sync::watch::Sender;
use tokio::task;
use tokio::time::Instant;
use tokio_metrics::{TaskMetrics, TaskMonitor};

use crate::ControllerChange;
use crate::monitoring::metrics::{IDLE_DURATION_METRIC, POLL_DURATION_METRIC, SCHEDULED_DURATION_METRIC};
use crate::ps_move::api::PsMoveApi;
use crate::ps_move::controller::PsMoveController;
use crate::ps_move::led_effects::{LedEffect, LedEffectKind};
use crate::tasks::{
    controller_update, controllers_list_update, effects_update, ip_discovery, mutations_handler,
};
use crate::tasks::models::EffectChange;

lazy_static! {
    static ref ON_STARTUP_EFFECT: LedEffect = LedEffect::new_expiring(
        LedEffectKind::new_timed_breathing(
            Hsv::from_components((270.0, 1.0, 0.001)),
            Duration::from_secs(3),
            0.3
        ),
        Duration::from_secs(3)
    );
}

pub async fn run_move(
    effect_rx: Receiver<EffectChange>,
    ctrl_tx: Sender<ControllerChange>,
    controllers: &Arc<Mutex<Vec<PsMoveController>>>,
) -> ShutdownCommand {
    let monitors = Monitors {
        effects_update: TaskMonitor::new(),
        controllers_list: TaskMonitor::new(),
        controller_update: TaskMonitor::new(),
    };

    let api = PsMoveApi::new();
    let shutdown_flag = Arc::new(AtomicBool::new(false));
    let initial_effect = Arc::new(Mutex::new(InitialLedState::from(*ON_STARTUP_EFFECT)));
    let (send, recv) = mpsc::channel::<()>(1);

    {
        let controllers = controllers.clone();
        let initial_effect = initial_effect.clone();

        tokio::spawn(mutations_handler::run(
            controllers,
            effect_rx,
            initial_effect,
        ));
    }

    tokio::spawn(monitors.effects_update.instrument(effects_update::run(
        controllers.clone(),
        initial_effect.clone(),
    )));

    {
        let controllers = controllers.clone();
        let shutdown_signal = ShutdownSignal::new(&send, &shutdown_flag);
        let monitor = monitors.controllers_list.clone();

        task::spawn_blocking(move || {
            Handle::current().block_on(monitor.instrument(controllers_list_update::run(
                controllers,
                api,
                shutdown_signal,
                initial_effect,
            )))
        });
    }

    {
        let controllers = controllers.clone();
        let shutdown_signal = ShutdownSignal::new(&send, &shutdown_flag);
        let monitor = monitors.controller_update.clone();

        task::spawn_blocking(move || {
            Handle::current().block_on(monitor.instrument(controller_update::run(
                controllers,
                ctrl_tx,
                shutdown_signal,
            )))
        });
    }

    tokio::spawn(ip_discovery::spawn());

    let frequency = Duration::from_secs(10);

    task::spawn(async move {
        let intervals = monitors.effects_update.intervals().zip(
            monitors
                .controllers_list
                .intervals()
                .zip(monitors.controller_update.intervals()),
        );

        for (effects_update, (controllers_list, controller_update)) in intervals {
            log_metrics("effects_update", effects_update);
            log_metrics("controllers_list", controllers_list);
            log_metrics("controller_update", controller_update);

            let _ = tokio::time::sleep(frequency).await;
        }
    });

    ShutdownCommand {
        flag: shutdown_flag,
        channel: recv,
    }
}

fn log_metrics(task_name: &str, metrics: TaskMetrics) {
    SCHEDULED_DURATION_METRIC.with_label_values(&[task_name]).observe(metrics.mean_scheduled_duration().as_secs_f64());
    POLL_DURATION_METRIC.with_label_values(&[task_name]).observe(metrics.mean_poll_duration().as_secs_f64());
    IDLE_DURATION_METRIC.with_label_values(&[task_name]).observe(metrics.mean_idle_duration().as_secs_f64());
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

pub struct InitialLedState {
    pub hsv: Hsv,
    pub effect: LedEffect,
}

impl InitialLedState {
    pub fn from(effect: LedEffect) -> InitialLedState {
        InitialLedState {
            hsv: effect.kind.get_initial_hsv(),
            effect,
        }
    }
}

struct Monitors {
    effects_update: TaskMonitor,
    controllers_list: TaskMonitor,
    controller_update: TaskMonitor,
}
