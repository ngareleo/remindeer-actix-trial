use std::sync::{ Arc, Mutex };

use actix_web::web;
use diesel::{ r2d2::ConnectionManager, PgConnection };
use r2d2::PooledConnection;

use crate::server::repository::user_repository::UserRespository;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub type AppUserRepository = web::Data<Arc<Mutex<UserRespository>>>;
