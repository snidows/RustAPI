#![allow(unused)]
use actix_web::{http::header::EXPECT, middleware as actix_middleware, web::Data, App, HttpServer};
mod api;
mod controllers;
mod models;
mod repository;
use api::users_api;
use sqlx::{postgres::PgPoolOptions, query, Pool, Postgres, Row};

use dotenvy::dotenv;
pub struct AppState {
    db: Pool<Postgres>,
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Error building connection Pool");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState { db: pool.clone() }))
            .wrap(actix_middleware::Logger::default())
            .service(users_api::create_user)
            .service(users_api::index)
        // always register default handler the last handler
        //
        // .default_service(web::to(middlewares::default::handler))
    })
    .bind(("127.0.0.1", 4444))?
    .run()
    .await
}
