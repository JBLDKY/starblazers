use actix_cors::Cors;
use actix_web::dev::Server;
use actix_web::http::header;
use actix_web::{web, App, HttpServer};
use service::database::db::DatabaseClient;
use service::filters::config_server;
use std::net::TcpListener;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    if dotenv::dotenv().is_err() {
        log::error!("Warning: Did not find .env file in current working directory!");
    }
    std::env::set_var("RUST_LOG", "debug");
    pretty_env_logger::init();

    let db = DatabaseClient::new().await;

    db.test().await;

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");

    let port = listener.local_addr().unwrap().port();
    dbg!(port);

    run(listener, db)?.await;

    Ok(())
}

fn run(listener: TcpListener, db_client: DatabaseClient) -> Result<Server, std::io::Error> {
    let db_client = web::Data::new(Arc::new(db_client));

    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .expose_headers(vec![header::AUTHORIZATION])
            .allowed_header(header::CONTENT_TYPE);

        App::new()
            .wrap(cors)
            .app_data(db_client.clone())
            .configure(config_server)
    })
    .listen(listener)?
    .run();

    Ok(server)
}
