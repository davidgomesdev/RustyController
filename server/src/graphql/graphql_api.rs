use std::io;
use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{
    App, Error, get,
    HttpResponse,
    HttpServer, middleware, Responder, route, web::{self, Data},
};
use actix_web::dev::Server;
use actix_web::web::Json;
use actix_web_lab::respond::Html;
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;
use tokio::sync::watch::Sender;

use crate::ps_move_api::LedEffect;

use super::schema::{Context, create_schema, Schema};

#[get("/graphiql")]
async fn graphiql() -> impl Responder {
    Html(graphiql_source("/graphql", None))
}

#[route("/graphql", method = "GET", method = "POST")]
async fn graphql(
    tx: Data<Arc<Sender<LedEffect>>>,
    schema: Data<Schema>,
    data: Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let ctx = Context {
        tx: tx.get_ref().to_owned(),
    };

    let res = data.execute(&schema, &ctx).await;

    Ok(HttpResponse::Ok().json(res))
}

pub async fn start(tx: Arc<Sender<LedEffect>>) -> io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(create_schema()))
            .app_data(Data::new(tx.to_owned()))
            .wrap(middleware::Logger::default())
            .wrap(Cors::permissive())
            .service(graphql)
            .service(graphiql)
    })
        .bind("0.0.0.0:8080")?
    .run()
    .await
}
