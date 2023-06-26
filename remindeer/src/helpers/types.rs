use diesel::{ r2d2::ConnectionManager, PgConnection };
use r2d2::PooledConnection;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;
