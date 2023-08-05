use lazy_static::lazy_static;
use prometheus::{Encoder, IntGauge, opts, register_int_gauge};
use warp::{Rejection, Reply};

lazy_static! {
    pub static ref CONNECTED_DEVICES_GAUGE: IntGauge =
        register_int_gauge!(opts!("connected_devices", "Number of devices connected"))
            .expect("Failed to create gauge");
}

pub async fn metrics_handler() -> Result<impl Reply, Rejection> {
    let encoder = prometheus::TextEncoder::new();

    let mut buffer = Vec::new();
    if let Err(e) = encoder.encode(&prometheus::gather(), &mut buffer) {
        eprintln!("could not encode prometheus metrics: {}", e);
    };
    let result = match String::from_utf8(buffer.clone()) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("prometheus metrics could not be from_utf8'd: {}", e);
            String::default()
        }
    };
    buffer.clear();

    Ok(result)
}
