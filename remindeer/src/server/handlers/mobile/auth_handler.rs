use std::sync::Arc;
use actix_web::{ Responder, post, web, get, HttpResponse };
use serde::{ Deserialize, Serialize };
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
    username: String,
    email: String,
}

#[derive(Deserialize, Serialize)]
pub struct UserExistStatus {
    username: bool,
    email: bool,
}

#[derive(Deserialize, Serialize)]
pub struct PhoneNumberExistStatus {
    exists: bool,
}

#[get("/user-details")]
pub async fn check_user_details_exists(
    app_user_repository: AppUserRepository,
    form: web::Form<PersonalDetailsForm>
) -> Result<impl Responder, UserRepositoryErrors> {
    let result = web
        ::block(
            move || -> Result<UserExistStatus, UserRepositoryErrors> {
                let repository = Arc::clone(&app_user_repository);
                let mut repository = repository
                    .lock()
                    .map_err(|_|
                        UserRepositoryErrors::create_external_error(
                            String::from("Error acquiring lock")
                        )
                    )?;
                let email = repository.email_exists(&form.email)?;
                let username = repository.username_exists(&form.username)?;

                Ok(UserExistStatus { email, username })
            }
        ).await
        .map_err(|_|
            UserRepositoryErrors::create_external_error(String::from("Error returning result"))
        )??;

    Ok(HttpResponse::Ok().json(result))
}

#[get("/phone-number/{phone_number}")]
pub async fn check_phone_number_exists(
    app_user_repository: AppUserRepository,
    query: web::Path<String>
) -> Result<impl Responder, UserRepositoryErrors> {
    let result = web
        ::block(
            move || -> Result<PhoneNumberExistStatus, UserRepositoryErrors> {
                let repository = Arc::clone(&app_user_repository);
                let mut repository = repository
                    .lock()
                    .map_err(|_|
                        UserRepositoryErrors::create_external_error(
                            String::from("Error acquiring lock")
                        )
                    )?;
                let exists = repository.phone_number_exists(&query.to_string())?;
                Ok(PhoneNumberExistStatus { exists })
            }
        ).await
        .map_err(|_|
            UserRepositoryErrors::create_external_error(String::from("Error returning result"))
        )??;

    Ok(HttpResponse::Ok().json(result))
}
