use actix_web::{
    get, post,
    web::{self, Json},
    HttpResponse, Responder, Result,
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct MyObj {
    name: String,
}
#[get("/{name}")]
async fn index(name: web::Path<String>) -> Result<impl Responder> {
    let obj = MyObj {
        name: name.to_string(),
    };
    Ok(web::Json(obj))
}

#[post("/")]
async fn post(info: Json<MyObj>) -> Result<impl Responder> {
    let obj = MyObj {
        name: info.name.to_string(),
    };
    let name = info.name.to_string();
    println!("{name}");
    Ok(web::Json(obj))
}
