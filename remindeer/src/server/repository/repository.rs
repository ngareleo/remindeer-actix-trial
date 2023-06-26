use std::error::Error;
use crate::helpers::types::{ DbPool, DbConnection };

pub trait RepositoryConfig<T> {
    fn get_pool(&self) -> DbPool;
    fn get_connection(&self) -> Result<DbConnection, Box<dyn Error>>;
}
