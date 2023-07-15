use serde::{ Serialize, Deserialize };
use diesel::prelude::*;
use uuid;

#[derive(Serialize, Deserialize, Debug, Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub name: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: chrono::NaiveDateTime,
    pub last_modified: chrono::NaiveDateTime,
    pub phone_number: String,
    pub unid: uuid::Uuid,
    pub photo: String,
}
