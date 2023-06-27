use std::{ error::Error, sync::{ Arc, Mutex } };
use actix_web::{ get, web, Responder, HttpResponse };
use crate::server::repository::user_repository::UserRespository;

#[get("/users")]
pub async fn get_users(
    repo: web::Data<Arc<Mutex<UserRespository>>>
) -> Result<impl Responder, Box<dyn Error>> {
    let users = web::block(move || {
        let repo = Arc::clone(&repo);
        let mut repository = repo.lock().unwrap();
        let users = repository.get_all_users().unwrap_or_else(|error| {
            dbg!("{}", error);
            vec![]
        });
        users
    }).await?;
    Ok(HttpResponse::Ok().json(users))
}
