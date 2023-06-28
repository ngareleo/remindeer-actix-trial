use crate::helpers::types::{ DbPool, DbConnection };

pub trait RepositoryConfig<E> {
    fn get_pool(&self) -> DbPool;
    fn get_connection(&self) -> Result<DbConnection, E>;
}
