use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();
    // 環境ごとに切り替える
    let url = "mysql://root@db/diesel_demo";
    // let url = env::var("DATABASE_URL").unwrap_or_else(|_| panic!("DATABASE_URL must be set"));
    MysqlConnection::establish(&url)
        .unwrap_or_else(|_| panic!(format!("Error connecting to {}", url)))
}
