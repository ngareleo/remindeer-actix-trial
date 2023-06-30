use crate::{
    server::models::user_model::User,
    server::repository::repository::RepositoryConfig,
    helpers::types::{ DbConnection, DbPool },
    schema::users,
};
use diesel::{ prelude::*, insert_into };
use serde::Deserialize;
use super::repository_errors::UserRepositoryErrors;

#[derive(Deserialize, Insertable, Debug)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub username: &'a str,
    pub password: &'a str,
    pub phone_number: &'a str,
}

pub struct UserRespository {
    pub pool: DbPool,
}

impl RepositoryConfig<UserRepositoryErrors> for UserRespository {
    fn get_connection(&self) -> Result<DbConnection, UserRepositoryErrors> {
        Ok(self.pool.get().map_err(|_| UserRepositoryErrors::ConnectionError)?)
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
        username: &str,
        phone_number: &str
    ) -> Result<User, UserRepositoryErrors> {
        let mut conn = self.get_connection()?;
        let new_user = NewUser { name, email, password, username, phone_number };
        let user = insert_into(users::table)
            .values(&new_user)
            .get_result(&mut conn)
            .map_err(|_| UserRepositoryErrors::UserInsertionError)?;
        Ok(user)
    }

    pub fn user_exists(&mut self, uname: &str, pass: &str) -> Result<User, UserRepositoryErrors> {
        let mut conn = self.get_connection()?;
        let user: User = users::table
            .filter(users::username.eq(uname))
            .select(User::as_select())
            .get_result(&mut conn)
            .map_err(|_| UserRepositoryErrors::UserNotFound)?;

        if user.password != pass {
            return Err(UserRepositoryErrors::IncorrectPassword);
        }

        Ok(user)
    }

    pub fn get_all_users(&mut self) -> Result<Vec<User>, UserRepositoryErrors> {
        let mut conn = self.get_connection()?;
        let results = users::table
            .limit(10)
            .select(User::as_select())
            .load(&mut conn)
            .map_err(|_| UserRepositoryErrors::UsersFetchingError)?;
        Ok(results)
    }

    pub fn email_exists(&mut self, email: &str) -> Result<bool, UserRepositoryErrors> {
        let mut conn = self.get_connection()?;
        let count = users::table
            .filter(users::email.eq(email))
            .count()
            .get_result(&mut conn)
            .optional()
            .map_err(|_| UserRepositoryErrors::DieselError)?;
        let is_exist = count.unwrap_or(0) >= 1;
        Ok(is_exist)
    }

    pub fn username_exists(&mut self, username: &str) -> Result<bool, UserRepositoryErrors> {
        let mut conn = self.get_connection()?;
        let count = users::table
            .filter(users::username.eq(username))
            .count()
            .get_result(&mut conn)
            .optional()
            .map_err(|_| UserRepositoryErrors::DieselError)?;
        let is_exist = count.unwrap_or(0) >= 1;
        Ok(is_exist)
    }

    pub fn phone_number_exists(
        &mut self,
        phone_number: &str
    ) -> Result<bool, UserRepositoryErrors> {
        let mut conn = self.get_connection()?;
        let count = users::table
            .filter(users::phone_number.eq(phone_number))
            .count()
            .get_result(&mut conn)
            .optional()
            .map_err(|_| UserRepositoryErrors::DieselError)?;
        let is_exist = count.unwrap_or(0) >= 1;
        Ok(is_exist)
    }
}
