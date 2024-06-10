use std::sync::Arc;

use service::{database::db::DatabaseClient, filters::config_server};

use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if dotenv::dotenv().is_err() {
        log::error!("Warning: Did not find .env file in current working directory!");
    }
    std::env::set_var("RUST_LOG", "debug");
    pretty_env_logger::init();

    let db = DatabaseClient::new().await;

    db.test().await;

    //let filters = all(Arc::new(db));

    //warp::serve(filters).run(([127, 0, 0, 1], 3030)).await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Arc::new(db.clone())))
            .configure(config_server)
    })
    .bind(("127.0.0.1", 3030))?
    .run()
    .await
    //http_server(Arc::new(db)).await
}
