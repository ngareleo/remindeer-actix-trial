use std::{ error::Error, sync::Arc };
use actix_web::{ get, web, Responder, HttpResponse };
use crate::helpers::types::AppUserRepository;

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
