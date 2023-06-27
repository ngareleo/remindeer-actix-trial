use std::sync::{ Mutex, Arc };
use actix_web::{ Responder, Error, HttpResponse, web, post };
use serde::Deserialize;
use crate::{ helpers::types::DbPool, server::repository::user_repository::UserRespository };

#[derive(Deserialize)]
struct LoginFormData {
    email: String,
    password: String,
}

#[post("/signin")]
pub async fn sign_in(
    pool: web::Data<DbPool>,
    form: web::Data<LoginFormData>
) -> Result<impl Responder, Error> {}

#[post("/signup")]
pub async fn sign_up(
    repository: web::Data<Arc<Mutex<UserRespository>>>
) -> Result<impl Responder, Error> {}
