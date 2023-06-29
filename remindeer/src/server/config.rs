use actix_web::{
    web::{ Data, self },
    guard,
    HttpServer,
    App,
    middleware::Logger,
    Error,
    error,
    HttpResponse,
};
use crate::{
    server::{
        handlers::{ api::users_handler, mobile::{ auth_handler, index_handler } },
        middleware::error_handling,
    },
    helpers::types::{ DbPool, AppUserRepository },
};

#[derive(Clone)]
pub struct AppConfig {
    pub port: u16,
    pub db_pool: Data<DbPool>,
    pub user_repository: AppUserRepository,
}

impl AppConfig {
    pub fn build(db_pool: Data<DbPool>, user_repository: AppUserRepository, port: u16) -> Self {
        AppConfig { db_pool, user_repository, port }
    }
}

pub async fn run(app_config: AppConfig) -> Result<(), Error> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    Ok(
        HttpServer::new(move || {
            let logger = Logger::default();
            let form_config = web::FormConfig
                ::default()
                .limit(4096)
                .error_handler(|err, _req| {
                    error::InternalError
                        ::from_response(err, HttpResponse::Conflict().finish())
                        .into()
                });
            App::new()
                .app_data(form_config)
                .app_data(app_config.user_repository.clone())
                .app_data(app_config.db_pool.clone())
                .wrap(logger)
                .service(index_handler::index)
                .service(users_handler::get_users)
                .service(users_handler::new_user)
                .service(auth_handler::login)
                .default_service(
                    web::route().guard(guard::Not(guard::Get())).to(error_handling::handle404)
                )
        })
            .bind(("127.0.0.1", app_config.port))?
            .run().await?
    )
}
