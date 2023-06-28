use derive_more::{ Display, Error };

pub trait RepositoryError {
    fn get_error_message(&self) -> &'static str;
}

#[derive(Debug, Display, Error)]
pub enum UserRepositoryErrors {
    #[display(fmt = "Incorrect password")] IncorrectPassword(),
    #[display(fmt = "User not found")] UserNotFound(),
    #[display(fmt = "Cannot get Postgres connection")] ConnectionError(),
}

impl RepositoryError for UserRepositoryErrors {
    fn get_error_message(&self) -> &'static str {
        match *self {
            UserRepositoryErrors::ConnectionError() => "cannot connect to database",
            UserRepositoryErrors::IncorrectPassword() => "incorrect password",
            UserRepositoryErrors::UserNotFound() => "user not found",
        }
    }
}
