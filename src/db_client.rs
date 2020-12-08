use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();
    let url = env::var("DATABASE_URL").unwrap_or_else(|_| panic!("DATABASE_URL must be set"));
    MysqlConnection::establish(&url)
        .unwrap_or_else(|_| panic!(format!("Error connecting to {}", url)))
}
