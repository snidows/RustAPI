use crate::controllers;
use crate::models::types;
use crate::AppState;
use actix_web::{
    body, get, post,
    web::{self, Data, Json},
    HttpResponse, Responder, Result,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use types::UserDTO;
use uuid::Uuid;
#[derive(Deserialize, Serialize)]
pub struct UserBody {
    pub name: String,
    pub street: String,
    pub city: String,
    pub state: String,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct UserSQL {
    pub id: i32,
    pub uuid: String,
    pub name: String,
    pub street: String,
    pub city: String,
    pub state: String,
}

#[get("/")]
pub async fn index(state: Data<AppState>) -> impl Responder {
    match sqlx::query_as::<_, UserSQL>("SELECT * FROM users")
        .fetch_all(&state.db)
        .await
    {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(error) => HttpResponse::InternalServerError().into(),
    }
}
#[post("/")]
pub async fn create_user(state: Data<AppState>, body: Json<UserBody>) -> Result<impl Responder> {
    let user: UserBody = body.into_inner();
    let uuid = Uuid::new_v4().to_string();
    match sqlx::query_as::<_, UserSQL>(
        "INSERT INTO users (uuid,name, street,city,state)
        VALUES ($1,$2,$3,$4,$5)
        RETURNING id,uuid,name,street,city,state",
    )
    .bind(uuid)
    .bind(user.name)
    .bind(user.street)
    .bind(user.city)
    .bind(user.state)
    .fetch_one(&state.db)
    .await
    {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(error) => Ok(HttpResponse::InternalServerError().json(format!("{:?}", error))),
    }
}
