use std::sync::Arc;

use futures::FutureExt as _;
use juniper_graphql_ws::ConnectionConfig;
use juniper_warp::playground_filter;
use juniper_warp::subscriptions::serve_graphql_ws;
use log::info;
use tokio::sync::Mutex;
use tokio::sync::watch::Sender;
use warp::{Filter, http::Response};

use crate::EffectChange;
use crate::ps_move::controller::PsMoveController;

use super::schema::{Context, create_schema};

pub async fn start(
    tx: Arc<Sender<EffectChange>>,
    controllers: Arc<Mutex<Vec<Box<PsMoveController>>>>,
) -> () {
    let log = warp::log("warp_subscriptions");
    let ctx = Context {
        tx: tx.clone(),
        controllers: controllers.clone(),
    };

    let homepage = warp::path::end().map(|| {
        Response::builder()
            .header("content-type", "text/html")
            .body("<html><h1>juniper_subscriptions demo</h1><div>visit <a href=\"/playground\">graphql playground</a></html>")
    });

    let qm_schema = create_schema();
    let qm_state = warp::any().map(move || ctx.clone());
    let qm_graphql_filter = juniper_warp::make_graphql_filter(qm_schema, qm_state.boxed());

    let root_node = Arc::new(create_schema());

    info!("Listening on 127.0.0.1:8080");

    let routes = (warp::path("subscriptions")
        .and(warp::ws())
        .map(move |ws: warp::ws::Ws| {
            let root_node = root_node.clone();
            let ctx = Context {
                tx: tx.clone(),
                controllers: controllers.clone(),
            };

            ws.on_upgrade(move |websocket| async move {
                serve_graphql_ws(
                    websocket,
                    root_node,
                    ConnectionConfig::new(ctx.clone()),
                )
                    .map(|r| {
                        if let Err(e) = r {
                            println!("Websocket error: {e}");
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
        .or(homepage)
        .with(log);

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await
}
