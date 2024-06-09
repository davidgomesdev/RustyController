use lazy_static::lazy_static;
use prometheus::{Encoder, HistogramVec, IntGauge, opts, register_int_gauge, register_histogram_vec, histogram_opts};
use warp::{Rejection, Reply};

lazy_static! {
    pub static ref CONNECTED_DEVICES_METRIC: IntGauge =
        register_int_gauge!(opts!("connected_devices", "Number of devices connected"))
            .expect("Failed to create connected devices metric");

    pub static ref SCHEDULED_DURATION_METRIC: HistogramVec =
        register_histogram_vec!(histogram_opts!("scheduled_duration", "Time it takes for a task to be executed by the scheduler"), &["task"])
            .expect("Failed to create scheduled duration metric");

    pub static ref POLL_DURATION_METRIC: HistogramVec =
        register_histogram_vec!(histogram_opts!("poll_duration", "Time a task takes to complete"), &["task"])
            .expect("Failed to create poll duration metric");

    pub static ref IDLE_DURATION_METRIC: HistogramVec =
        register_histogram_vec!(histogram_opts!("idle_duration", "The time a task spent idle (i.e. sleeping)"), &["task"])
            .expect("Failed to create idle duration metric");
}

pub async fn metrics_handler() -> Result<impl Reply, Rejection> {
    let encoder = prometheus::TextEncoder::new();

    let mut buffer = Vec::new();
    if let Err(e) = encoder.encode(&prometheus::gather(), &mut buffer) {
        eprintln!("could not encode prometheus metrics: {}", e);
    };
    let result = String::from_utf8(buffer.clone()).unwrap_or_else(|e| {
        eprintln!("prometheus metrics could not be from_utf8'd: {}", e);
        String::default()
    });
    buffer.clear();

    Ok(result)
}
