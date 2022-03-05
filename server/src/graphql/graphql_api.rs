use std::io;
use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{middleware, web::{self, Data}, App, Error, HttpResponse, HttpServer};
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

use super::schema::{create_schema, Schema};

async fn graphiql() -> HttpResponse {
    let html = graphiql_source("http://127.0.0.1:8080/graphql", None);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

async fn graphql(
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let res = data.execute_sync(&st, &());
        println!("{}", res.is_ok());
        serde_json::to_string(&res)
    })
    .await?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body::<String>(user.unwrap_or("default".to_owned())))
}

pub async fn start() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // Create Juniper schema
    let schema = std::sync::Arc::new(create_schema());

    // Start http server
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone()))
            .wrap(middleware::Logger::default())
            .wrap(
                Cors::permissive(),
            )
            .service(web::resource("/graphql").route(web::post().to(graphql)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
