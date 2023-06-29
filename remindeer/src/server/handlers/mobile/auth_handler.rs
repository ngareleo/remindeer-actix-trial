use std::sync::Arc;

use actix_web::{ get, Responder, post, web, HttpResponse };
use serde::Deserialize;

use crate::{
    helpers::types::AppUserRepository,
    server::{ repository::repository_errors::UserRepositoryErrors, models::user_model::User },
};

#[derive(Deserialize)]
pub struct LoginFormData {
    username: String,
    password: String,
}

#[post("/login")]
pub async fn login(
    app_user_repository: AppUserRepository,
    form: web::Form<LoginFormData>
) -> Result<impl Responder, UserRepositoryErrors> {
    let result = web
        ::block(
            move || -> Result<User, UserRepositoryErrors> {
                let repository = Arc::clone(&app_user_repository);
                let mut repository = repository
                    .lock()
                    .map_err(|_|
                        UserRepositoryErrors::create_external_error(
                            String::from("Error acquiring lock")
                        )
                    )?;
                repository.user_exists(&form.username, &form.password)
            }
        ).await
        .map_err(|_|
            UserRepositoryErrors::create_external_error(String::from("Block return error"))
        )?;

    result.map(|user| Ok(HttpResponse::Ok().json(user)))?
}

#[derive(Deserialize)]
pub struct PersonalDetailsForm {
    name: String,
    username: String,
    email: String,
}

#[get("/valid-user-details")]
pub async fn check_user_details_validity(
    app_user_repository: AppUserRepository,
    form: web::Form<PersonalDetailsForm>
) -> Result<impl Responder, UserRepositoryErrors> {
    let result = web
        ::block(
            move || -> Result<Option<&'static str>, UserRepositoryErrors> {
                let repository = Arc::clone(&app_user_repository);
                let mut repository = repository
                    .lock()
                    .map_err(|_|
                        UserRepositoryErrors::create_external_error(
                            String::from("Error acquiring lock")
                        )
                    )?;
                repository.user_exists(&form.username, &form.password)
            }
        ).await
        .map_err(|_|
            UserRepositoryErrors::create_external_error(String::from("Error returning result"))
        )?;

    Ok(HttpResponse::Ok().json(value))
}
