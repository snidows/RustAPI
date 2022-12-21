use crate::controllers::{self, user_controller::insert_user};
use crate::models::types;
use crate::AppState;
use actix_web::{
    body, get, post,
    web::{self, Data, Json},
    HttpResponse, Responder, Result,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use types::*;
use uuid::Uuid;

#[get("/")]
pub async fn index(state: Data<AppState>) -> impl Responder {
    match sqlx::query_as::<_, UserSQL>("SELECT * FROM users")
        .fetch_all(&state.db)
        .await
    {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(error) => HttpResponse::InternalServerError().json(format!("{:?}", error)),
    }
}
#[post("/")]
pub async fn create_user(state: Data<AppState>, body: Json<UserBody>) -> Result<impl Responder> {
    return Ok(insert_user(
        body.name.to_string(),
        body.street.to_string(),
        body.city.to_string(),
        body.state.to_string(),
        state,
        body,
    )
    .await);
}
