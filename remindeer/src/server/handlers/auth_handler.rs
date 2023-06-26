use actix_web::{ get, Responder, Error, HttpResponse, web };
use serde::Deserialize;

use crate::helpers::types::DbPool;

#[derive(Deserialize)]
struct LoginFormData {
    email: String,
    password: String,
}

#[get("/login")]
pub async fn login(
    pool: web::Data<DbPool>,
    form: web::Data<LoginFormData>
) -> Result<impl Responder, Error> {
    Ok(HttpResponse::Ok().body(String::from("Hello Leo")))
}
