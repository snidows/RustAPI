use crate::models::types::{UserBody, UserSQL};
use crate::repository::postgres;
use crate::AppState;
use actix_web::{
    body, get, post,
    web::{self, Data, Json},
    HttpResponse, Responder, Result,
};
use uuid::Uuid;

pub async fn show_users(state: Data<AppState>) -> HttpResponse {
    let response = match sqlx::query_as::<_, UserSQL>("SELECT * FROM users")
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

pub async fn update_user(
    app: Data<AppState>,
    uuid: String,
    name: String,
    street: String,
    city: String,
    state: String,
) -> HttpResponse {
    return match sqlx::query_as::<_, UserSQL>(
        "UPDATE users SET name=$1,street = $2,city =$3,state=$4 where uuid =$5 returning  id,uuid,name,street ,city ,state",
    )
    .bind(name)
    .bind(street)
    .bind(city)
    .bind(state)
    .bind(uuid)
    .fetch_one(&app.db)
    .await
    {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(error) => HttpResponse::InternalServerError().json(format!("{:?}", error)),
    };
}

pub async fn delete_user(app: Data<AppState>, uuid: String) -> HttpResponse {
    return match sqlx::query_as::<_, UserSQL>("delete from users where uuid=$1")
        .bind(uuid)
        .fetch_one(&app.db)
        .await
    {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(error) => HttpResponse::InternalServerError().json(format!("{:?}", error)),
    };
}

pub async fn get_user(app: Data<AppState>, uuid: String) -> HttpResponse {
    return match sqlx::query_as::<_, UserSQL>("select* from users where uuid=$1")
        .bind(uuid)
        .fetch_one(&app.db)
        .await
    {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(error) => HttpResponse::InternalServerError().json(format!("{:?}", error)),
    };
}
