mod server;
mod helpers;
mod schema;

use std::{ env, process };
use std::sync::{ Arc, Mutex };
use actix_web::web;
use diesel::PgConnection;
use diesel::r2d2::{ self, ConnectionManager };
use dotenvy::dotenv;
use helpers::types::DbPool;
use server::config::{ run, AppConfig };
use server::repository::user_repository::UserRespository;

#[actix_web::main]
async fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").unwrap_or_else(|error| {
        dbg!("{}", error);
        process::exit(1)
    });

    let manager = ConnectionManager::<PgConnection>::new(&database_url);
    let pool: DbPool = r2d2::Pool
        ::builder()
        .build(manager)
        .unwrap_or_else(|error| {
            dbg!("{}", error);
            process::exit(1)
        });

    let user_repo = Arc::new(Mutex::new(UserRespository::new(pool.clone())));

    let app_config = AppConfig::build(
        web::Data::new(pool.clone()),
        web::Data::new(user_repo.clone()),
        8080
    );

    match run(app_config.clone()).await {
        Ok(_) => println!("Server running on http://localhost:{} closed", app_config.port),
        Err(error) => {
            dbg!("{}", error);
            process::exit(1)
        }
    }
}
