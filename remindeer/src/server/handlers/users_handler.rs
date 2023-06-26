use std::error::Error;

use actix_web::{ get, web, Responder, HttpResponse };

use crate::{
    server::repository::{ user_repository::UserRespository, repository::RepositoryConfig },
};

#[get("/users")]
pub async fn get_users(
    repository: web::Data<UserRespository>
) -> Result<impl Responder, Box<dyn Error>> {
    let users = web::block(move || {
        let users = repository.get_all_users().unwrap_or_else(|error| {
            dbg!("{}", error);
            vec![]
        });
        users
    }).await?;
    Ok(HttpResponse::Ok().json(users))
}
