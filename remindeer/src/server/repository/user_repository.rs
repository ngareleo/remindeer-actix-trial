use std::error::Error;
use crate::{
    server::models::user_model::User,
    server::repository::repository::RepositoryConfig,
    helpers::types::{ DbConnection, DbPool },
    schema::users,
};
use diesel::{ prelude::*, insert_into };
use serde::Deserialize;

#[derive(Deserialize, Insertable, Debug)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub username: &'a str,
    pub password: &'a str,
}

pub struct UserRespository {
    pub pool: DbPool,
}

impl RepositoryConfig for UserRespository {
    fn get_connection(&self) -> Result<DbConnection, Box<dyn Error>> {
        Ok(self.pool.get()?)
    }

    fn get_pool(&self) -> DbPool {
        self.pool.clone()
    }
}

impl UserRespository {
    pub fn new(pool: DbPool) -> Self {
        UserRespository { pool }
    }

    pub fn create_user<'a>(
        &mut self,
        name: &str,
        email: &str,
        password: &str,
        username: &str
    ) -> Result<User, Box<dyn Error>> {
        let mut conn = self.get_connection()?;
        let new_user = NewUser { name, email, password, username };
        let user = insert_into(users::table).values(&new_user).get_result(&mut conn)?;
        Ok(user)
    }

    pub fn user_exists(&mut self, uname: &str, pass: &str) -> Result<Option<User>, Box<dyn Error>> {
        let mut conn = self.get_connection()?;
        let user = users::table
            .filter(users::username.eq(uname))
            .select(User::as_select())
            .get_result(&mut conn)?;

        Ok(Some(user))
    }

    pub fn get_all_users(&mut self) -> Result<Vec<User>, Box<dyn Error>> {
        let mut conn = self.get_connection()?;
        let results = users::table
            .limit(10)
            .select(User::as_select())
            .load(&mut conn)
            .expect("Error loading users");
        Ok(results)
    }
}
