use crate::controllers::{health_check, login};
use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(login::login));
    cfg.route("/health_check", web::get().to(health_check::health_check));
}
