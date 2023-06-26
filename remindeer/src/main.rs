mod server;
mod helpers;
mod schema;

use std::env;
use actix_web::web::Data;
use actix_web::{ HttpServer, middleware::Logger, guard, web, App };
use diesel::PgConnection;
use diesel::r2d2::{ self, ConnectionManager };
use dotenvy::dotenv;
use helpers::types::DbPool;
use server::handlers::*;
use server::middleware::error_handling;
use server::repository::user_repository::UserRespository;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("Missing database url");

    let manager = ConnectionManager::<PgConnection>::new(&database_url);
    let pool: Data<DbPool> = Data::new(
        r2d2::Pool::builder().build(manager).expect("Failed to create pool")
    );

    let userRepository = UserRespository

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .app_data(pool.clone())
            .wrap(logger)
            .service(index_handler::index)
            .service(users_handler::get_users)
            .default_service(
                web::route().guard(guard::Not(guard::Get())).to(error_handling::handle404)
            )
    })
        .bind(("127.0.0.1", 8080))?
        .run().await
}
