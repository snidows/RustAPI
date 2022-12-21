use crate::controllers::user_controller;
use crate::models::types;
use crate::AppState;
use actix_web::{
    body, delete, get, post, put,
    web::{self, Data, Json},
    HttpResponse, Responder, Result,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use types::*;
use uuid::Uuid;

#[get("/users")]
pub async fn show_users(state: Data<AppState>) -> impl Responder {
    return user_controller::show_users(state).await;
}

#[post("/users")]
pub async fn create_user(state: Data<AppState>, body: Json<UserBody>) -> Result<impl Responder> {
    return Ok(user_controller::create_user(
        body.name.to_string(),
        body.street.to_string(),
        body.city.to_string(),
        body.state.to_string(),
        state,
        body,
    )
    .await);
}

#[put("/users/{uuid}")]
pub async fn update_user(
    app: Data<AppState>,
    path: web::Path<String>,
    body: Json<UserBody>,
) -> impl Responder {
    return user_controller::update_user(
        app,
        path.to_string(),
        body.name.to_string(),
        body.street.to_string(),
        body.city.to_string(),
        body.state.to_string(),
    )
    .await;
}

#[delete("/users/{uuid}")]
pub async fn delete_user(app: Data<AppState>, path: web::Path<String>) -> impl Responder {
    return user_controller::delete_user(app, path.to_string()).await;
}

#[get("/users/{uuid}")]
pub async fn get_user(app: Data<AppState>, path: web::Path<String>) -> impl Responder {
    return user_controller::get_user(app, path.to_string()).await;
}
