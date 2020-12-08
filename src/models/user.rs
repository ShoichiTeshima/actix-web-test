use diesel::Queryable;
use serde::Serialize;

#[derive(Queryable, Debug, Serialize)]
pub struct User {
    pub id: i64,
    pub family_name: String,
    pub first_name: String,
    pub mail_address: String,
    pub password: String,
    pub phone_number: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
