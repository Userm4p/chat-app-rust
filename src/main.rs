use actix_web::{  App, HttpServer, web };

use chat::infra;
use chat::config::read_config;
use chat::modules::music;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let config = read_config();

    HttpServer::new(|| {
        App::new()
        .service(web::scope("/api").configure(infra::routes::config).configure(music::infra::endpoints::config))
    })
    .bind((config.host, config.port))?
    .run()
    .await
}