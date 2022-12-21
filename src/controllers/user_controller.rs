use crate::models::types::{UserBody, UserSQL};
use crate::repository::postgres;
use crate::AppState;
use actix_web::{
    body, get, post,
    web::{self, Data, Json},
    HttpResponse, Responder, Result,
};
use uuid::Uuid;

pub async fn insert_user(
    name: String,
    street: String,
    city: String,
    state: String,
    app: Data<AppState>,
    body: Json<UserBody>,
) -> HttpResponse {
    let user: UserBody = body.into_inner();
    let uuid: String = Uuid::new_v4().to_string();
    let response = match sqlx::query_as::<_, UserSQL>(
        "INSERT INTO users (uuid,name, street,city,state)
        VALUES ($1,$2,$3,$4,$5)
        RETURNING id,uuid,name,street,city,state",
    )
    .bind(uuid)
    .bind(name)
    .bind(street)
    .bind(city)
    .bind(state)
    .fetch_one(&app.db)
    .await
    {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(error) => HttpResponse::InternalServerError().json(format!("{:?}", error)),
    };
    return response;
}

pub async fn update_user() {}

pub async fn delete_user() {}

pub async fn get_user() {}

pub async fn index_users() {}
