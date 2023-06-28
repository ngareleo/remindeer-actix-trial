use std::sync::Arc;
use actix_web::{ Responder, web, post, HttpResponse };
use serde::Deserialize;
use crate::{
    helpers::types::AppUserRepository,
    server::{ models::user_model::User, repository::repository_errors::UserRepositoryErrors },
};

#[derive(Deserialize)]
pub struct LoginFormData {
    username: String,
    password: String,
}

#[derive(Deserialize, Debug)]
pub struct SignUpFormData {
    name: String,
    email: String,
    password: String,
    username: String,
}

#[post("/signin")]
pub async fn sign_in(
    app_user_repository: AppUserRepository,
    form: web::Form<LoginFormData>
) -> Result<impl Responder, actix_web::Error> {
    let result = web::block(
        move || -> Result<Option<User>, UserRepositoryErrors> {
            let repository = Arc::clone(&app_user_repository);
            let mut repository = repository
                .lock()
                .map_err(|_|
                    UserRepositoryErrors::create_external_error(
                        String::from("Error acquiring lock")
                    )
                )?;
            Ok(repository.user_exists(&form.username, &form.password)?)
        }
    ).await?;

    match result {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(message) => {
            dbg!("{}", message);
            Ok(HttpResponse::Ok().body("Problem occurred"))
        }
    }
}

#[post("/signup")]
pub async fn sign_up(
    app_user_repository: AppUserRepository,
    form: web::Form<SignUpFormData>
) -> Result<impl Responder, actix_web::Error> {
    let result = web::block(
        move || -> Result<User, UserRepositoryErrors> {
            let repository = Arc::clone(&app_user_repository);
            let mut repository = repository
                .lock()
                .map_err(|_|
                    UserRepositoryErrors::create_external_error(
                        String::from("Error acquiring lock")
                    )
                )?;
            Ok(repository.create_user(&form.name, &form.email, &form.password, &form.username)?)
        }
    ).await?;

    match result {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(message) => {
            dbg!("{}", message);
            Ok(HttpResponse::Ok().body("User not created"))
        }
    }
}
