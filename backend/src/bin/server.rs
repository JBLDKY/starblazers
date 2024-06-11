use std::sync::Arc;

use actix_cors::Cors;
use service::{database::db::DatabaseClient, filters::config_server};

use actix_web::{http::header, web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if dotenv::dotenv().is_err() {
        log::error!("Warning: Did not find .env file in current working directory!");
    }
    std::env::set_var("RUST_LOG", "debug");
    pretty_env_logger::init();

    let db = DatabaseClient::new().await;

    db.test().await;

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .expose_headers(vec![header::AUTHORIZATION])
            .allowed_header(header::CONTENT_TYPE);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(Arc::new(db.clone())))
            .configure(config_server)
    })
    .bind(("127.0.0.1", 3030))?
    .run()
    .await
}
