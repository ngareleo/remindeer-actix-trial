use actix_web::{ get, Error };

#[get("/")]
pub async fn index() -> Result<String, Error> {
    Ok(String::from("Hello from remindeer"))
}
