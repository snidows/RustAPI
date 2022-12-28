use serde::{Deserialize, Serialize};
use sqlx::{
    types::chrono::{DateTime, Utc},
    FromRow,
};

#[derive(Serialize, Deserialize)]
pub struct UserDTO {
    pub name: String,
    pub street: String,
    pub city: String,
    pub state: String,
}
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

#[derive(Serialize, Deserialize, FromRow)]
pub struct UserInput {
    pub id: i32,
    pub uuid: String,
    pub name: String,
    pub street: String,
    pub city: String,
    pub state: String,
    // pub date: DateTime<Utc>,
}
