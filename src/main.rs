use actix_web::{get, App, HttpServer, Responder};
use actix_web_test::routes;

#[get("/health_check")]
async fn health_check() -> impl Responder {
    println!("eiya");
    format!("okdesu")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(routes::routes))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
