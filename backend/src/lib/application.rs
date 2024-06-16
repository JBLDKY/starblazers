use actix::{Actor, Addr};
use actix_cors::Cors;
use actix_web::dev::Server;
use actix_web::http::header::{self};
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use std::net::TcpListener;
use std::sync::Arc;
use std::time::Instant;

use crate::configuration::Settings;
use crate::database::db::DatabaseClient;
use crate::routes::config_server;
use crate::websocket::{LobbyServer, WsLobbySession};

pub struct Application {
    server: Server,
    port: u16,
}

impl Application {
    pub async fn build(settings: Settings) -> Result<Self, std::io::Error> {
        let address = format!(
            "{}:{}",
            settings.application.host, settings.application.port
        );
        let listener = TcpListener::bind(address).expect("Failed to bind to random port");
        let port = listener.local_addr().unwrap().port();

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

async fn lobby_websocket(
    req: HttpRequest,
    stream: web::Payload,
    srv: actix_web::web::Data<Addr<LobbyServer>>,
) -> Result<HttpResponse, actix_web::Error> {
    ws::start(
        WsLobbySession {
            id: 0,
            hb: Instant::now(),
            lobby: "main".to_owned(),
            name: Some("name".to_string()),
            addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    )
}

fn run(listener: TcpListener, db_client: DatabaseClient) -> Result<Server, std::io::Error> {
    let db_client = web::Data::new(Arc::new(db_client));

    let lobby_server = web::Data::new(LobbyServer::new().start());

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
            .app_data(lobby_server.clone())
            .configure(config_server)
            .route("/lobby", web::get().to(lobby_websocket))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
