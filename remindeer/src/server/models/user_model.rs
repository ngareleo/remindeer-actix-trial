use serde::{ Serialize, Deserialize };
use diesel::prelude::*;

// the structure of the model should be similar to schema.rs
#[derive(Serialize, Deserialize, Debug, Queryable, Selectable, PartialEq)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: uuid::Uuid,
    pub name: String,
    pub username: String,
    pub photo: String,
    pub email: String,
    pub password: String,
    pub phone_number: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
