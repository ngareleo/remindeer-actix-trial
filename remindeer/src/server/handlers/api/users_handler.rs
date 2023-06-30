use std::{ error::Error, sync::Arc };
use actix_web::{ get, web, put, Responder, HttpResponse };
use serde::Deserialize;
use crate::{
    helpers::types::AppUserRepository,
    server::{ repository::repository_errors::UserRepositoryErrors, models::user_model::User },
};

#[derive(Deserialize, Debug)]
pub struct SignUpFormData {
    name: String,
    email: String,
    password: String,
    username: String,
    phone_number: String,
}

#[put("/user")]
pub async fn new_user(
    app_user_repository: AppUserRepository,
    form: web::Form<SignUpFormData>
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
                Ok(
                    repository.create_user(
                        &form.name,
                        &form.email,
                        &form.password,
                        &form.username,
                        &form.phone_number
                    )?
                )
            }
        ).await
        .map_err(|_|
            UserRepositoryErrors::create_external_error(String::from("Block return error"))
        )?;
    result.map(|user| Ok(HttpResponse::Ok().json(user)))?
}

#[get("/users")]
pub async fn get_users(
    app_user_repository: AppUserRepository
) -> Result<impl Responder, Box<dyn Error>> {
    let users = web::block(move || {
        let repository = Arc::clone(&app_user_repository);
        let mut repository = repository.lock().unwrap();
        let users = repository.get_all_users().unwrap_or_else(|error| {
            dbg!("{}", error);
            vec![]
        });
        users
    }).await?;
    Ok(HttpResponse::Ok().json(users))
}
