use crate::db_client::establish_connection;
use crate::diesel::RunQueryDsl;
use crate::models::user::User;
use crate::schema::users::dsl::*;
use actix_web::{HttpResponse, Responder};

pub async fn login() -> impl Responder {
    let conn = establish_connection();
    let res = users.load::<User>(&conn).expect("load err");
    HttpResponse::Ok().body(serde_json::to_string(&res).unwrap())
}
