use std::time::SystemTime;

use crate::models::types::{UserBody, UserSQL};
use crate::repository::postgres;
use crate::AppState;
use actix_web::{
    body, get, post,
    web::{self, Data, Json},
    HttpResponse, Responder, Result,
};
use sqlx::types::chrono::{DateTime, Utc};
use uuid::Uuid;

pub async fn show_users(state: Data<AppState>) -> HttpResponse {
    let response =
        match sqlx::query_as::<_, UserSQL>("SELECT * FROM users where deleted_at is null")
            .fetch_all(&state.db)
            .await
        {
            Ok(user) => HttpResponse::Ok().json(user),
            Err(error) => HttpResponse::InternalServerError().json(format!("{:?}", error)),
        };
    return response;
}
pub async fn create_user(
    name: String,
    street: String,
    city: String,
    state: String,
    date: DateTime<Utc>,
    app: Data<AppState>,
    body: Json<UserBody>,
) -> HttpResponse {
    let user: UserBody = body.into_inner();
    let uuid: String = Uuid::new_v4().to_string();
    println!("{:?}", date);
    let response = match sqlx::query_as::<_, UserSQL>(
        "INSERT INTO users (uuid,name, street,city,state,created_at,updated_at)
        VALUES ($1,$2,$3,$4,$5,$6,$6)
        RETURNING id,uuid,name,street,city,state",
    )
    .bind(uuid)
    .bind(name)
    .bind(street)
    .bind(city)
    .bind(state)
    .bind(date)
    .fetch_one(&app.db)
    .await
    {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(error) => HttpResponse::InternalServerError().json(format!("{:?}", error)),
    };
    return response;
}

pub async fn update_user(
    app: Data<AppState>,
    uuid: String,
    name: String,
    street: String,
    city: String,
    state: String,
) -> HttpResponse {
    let date = chrono::offset::Utc::now();
    return match sqlx::query_as::<_, UserSQL>(
        "UPDATE users SET name=$1,street = $2,city =$3,state=$4,updated_at=$6 where uuid =$5 returning  id,uuid,name,street ,city ,state",
    )
    .bind(name)
    .bind(street)
    .bind(city)
    .bind(state)
    .bind(uuid)
    .bind(date)
    .fetch_one(&app.db)
    .await
    {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(error) => HttpResponse::InternalServerError().json(format!("{:?}", error)),
    };
}

pub async fn delete_user(app: Data<AppState>, uuid: String) -> HttpResponse {
    let date = chrono::offset::Utc::now();
    return match sqlx::query(
        "UPDATE users SET deleted_at=$2 where uuid =$1 and deleted_at is NULL returning ID",
    )
    .bind(uuid)
    .bind(date)
    .fetch_one(&app.db)
    .await
    {
        Ok(_) => HttpResponse::Ok().body("DELETED"),
        Err(error) => HttpResponse::InternalServerError().json(format!("{:?}", error)),
    };
}

pub async fn get_user(app: Data<AppState>, uuid: String) -> HttpResponse {
    return match sqlx::query_as::<_, UserSQL>(
        "select * from users where uuid=$1 and deleted_at is NULL",
    )
    .bind(uuid)
    .fetch_one(&app.db)
    .await
    {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(error) => HttpResponse::InternalServerError().json(format!("{:?}", error)),
    };
}
