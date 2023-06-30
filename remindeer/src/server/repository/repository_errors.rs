use derive_more::{ Display, Error };
use actix_web::{ error, http::StatusCode, HttpResponse };

pub trait RepositoryError {
    fn get_error_message(&self) -> &'static str;
}

#[derive(Debug, Display, Error)]
pub enum UserRepositoryErrors {
    #[display(fmt = "Incorrect password")] IncorrectPassword,
    #[display(fmt = "User not found")] UserNotFound,
    #[display(fmt = "Cannot get Postgres connection")] ConnectionError,
    #[display(fmt = "Error occurred while fetching users")] UsersFetchingError,
    #[display(fmt = "Error occurred while inserting users")] UserInsertionError,
    #[display(fmt = "{}", message)] ExternalError {
        message: String,
    },
    #[display(fmt = "Diesel error")] DieselError,
}

impl UserRepositoryErrors {
    pub fn create_external_error(message: String) -> Self {
        Self::ExternalError { message }
    }
}

impl error::ResponseError for UserRepositoryErrors {
    fn error_response(&self) -> HttpResponse {
        dbg!("[error] {}", self.to_string());
        HttpResponse::build(self.status_code()).body("Error occurred")
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            UserRepositoryErrors::UserNotFound | UserRepositoryErrors::IncorrectPassword =>
                StatusCode::BAD_REQUEST,
            | UserRepositoryErrors::ConnectionError
            | UserRepositoryErrors::UserInsertionError
            | UserRepositoryErrors::UsersFetchingError
            | _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
