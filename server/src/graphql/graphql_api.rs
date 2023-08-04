use std::sync::Arc;

use futures::FutureExt as _;
use juniper_graphql_ws::ConnectionConfig;
use juniper_warp::playground_filter;
use juniper_warp::subscriptions::serve_graphql_ws;
use tokio::sync::broadcast::Sender;
use tokio::sync::Mutex;
use tokio::sync::watch::Receiver;
use warp::{Filter, http::Response};

use crate::{ControllerChange, EffectChange};
use crate::metrics::metrics::metrics_handler;
use crate::ps_move::controller::PsMoveController;

use super::schema::{Context, create_schema};

pub async fn start(
    effect_tx: Arc<Sender<EffectChange>>,
    ctrl_rx: Mutex<Receiver<ControllerChange>>,
    controllers: Arc<Mutex<Vec<PsMoveController>>>,
) {
    let log = warp::log("warp_subscriptions");
    let ctrl_rx_arc = Arc::new(ctrl_rx);
    let qm_ctx = Context {
        effect_tx: effect_tx.clone(),
        ctrl_rx: ctrl_rx_arc.clone(),
        controllers: controllers.clone(),
    };

    let homepage = warp::path::end().map(|| {
        Response::builder()
            .header("content-type", "text/html")
            .body("<html><h1>RustyController</h1><div>Visit <a href=\"/playground\">Playground</a></html>")
    });

    let qm_schema = create_schema();
    let qm_state = warp::any().map(move || qm_ctx.clone());
    let qm_graphql_filter = juniper_warp::make_graphql_filter(qm_schema, qm_state.boxed());

    let root_node = Arc::new(create_schema());

    tracing::info!("Listening on 0.0.0.0:8080");

    let routes = (warp::path("subscriptions")
        .and(warp::ws())
        .map(move |ws: warp::ws::Ws| {
            let root_node = root_node.clone();
            let ctx = Context {
                effect_tx: effect_tx.clone(),
                ctrl_rx: ctrl_rx_arc.clone(),
                controllers: controllers.clone(),
            };

            ws.on_upgrade(move |websocket| async move {
                serve_graphql_ws(websocket, root_node, ConnectionConfig::new(ctx.clone()))
                    .map(|r| {
                        if let Err(e) = r {
                            tracing::error!("Websocket error: {e}");
                        }
                    })
                    .await
            })
        }))
        .map(|reply| warp::reply::with_header(reply, "Sec-WebSocket-Protocol", "graphql-ws"))
        .or(warp::post()
            .and(warp::path("graphql"))
            .and(qm_graphql_filter))
        .or(warp::get()
            .and(warp::path("playground"))
            .and(playground_filter("/graphql", Some("/subscriptions"))))
        .or(warp::get().and(warp::path("metrics")).and_then(metrics_handler))
        .or(homepage)
        .with(log);

    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await
}
