use std::sync::{ Arc, Mutex };

use actix_web::{ web::{ Data, self }, guard, HttpServer, App, middleware::Logger, Error };

use crate::helpers::types::DbPool;

use super::{
    repository::user_repository::UserRespository,
    handlers::{ index_handler, users_handler },
    middleware::error_handling,
};

#[derive(Clone)]
pub struct AppConfig {
    pub port: u16,
    pub db_pool: Data<DbPool>,
    pub user_repository: Data<Arc<Mutex<UserRespository>>>,
}

impl AppConfig {
    pub fn build(
        db_pool: Data<DbPool>,
        user_repository: Data<Arc<Mutex<UserRespository>>>,
        port: u16
    ) -> Self {
        AppConfig { db_pool, user_repository, port }
    }
}

pub async fn run_server(app_config: AppConfig) -> Result<(), Error> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    Ok(
        HttpServer::new(move || {
            let logger = Logger::default();
            App::new()
                .app_data(app_config.user_repository.clone())
                .app_data(app_config.db_pool.clone())
                .wrap(logger)
                .service(index_handler::index)
                .service(users_handler::get_users)
                .default_service(
                    web::route().guard(guard::Not(guard::Get())).to(error_handling::handle404)
                )
        })
            .bind(("127.0.0.1", app_config.port))?
            .run().await?
    )
}
