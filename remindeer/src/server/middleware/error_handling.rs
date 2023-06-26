use actix_web::Error;

pub async fn handle404() -> Result<String, Error> {
    Ok(format!("Resource not found"))
}
