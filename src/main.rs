use actix_web::{web, App, HttpServer};
mod api;
mod models;
mod repository;
mod routes;

use routes::users;
// use serde::{Deserialize, Serialize};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(users::index).service(users::post))
        .bind(("127.0.0.1", 4444))?
        .run()
        .await
}
