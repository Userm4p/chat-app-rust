use actix_web::{get, web, Responder, HttpResponse};

#[get("/version")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Version 0.1.0")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
}