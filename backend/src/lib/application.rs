use actix_cors::Cors;
use actix_web::dev::Server;
use actix_web::http::header;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;
use std::sync::Arc;

use crate::database::db::DatabaseClient;
use crate::filters::config_server;

pub struct Application {
    server: Server,
    port: u16,
}

impl Application {
    pub async fn build(host: &str, port: &str) -> Result<Self, std::io::Error> {
        let address = format!("{}:{}", host, port);
        let listener = TcpListener::bind(address).expect("Failed to bind to random port");
        let port = listener.local_addr().unwrap().port();
        dbg!(port);

        let db = DatabaseClient::new().await;

        let server = run(listener, db)?;

        Ok(Self { server, port })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn start(self) -> Result<(), std::io::Error> {
        self.server.await
    }
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
