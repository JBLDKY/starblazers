use std::sync::Arc;

use service::database::db::DatabaseClient;
use service::filters::all;

#[tokio::main]
async fn main() {
    if dotenv::dotenv().is_err() {
        log::error!("Warning: Did not find .env file in current working directory!");
    }
    std::env::set_var("RUST_LOG", "debug");
    pretty_env_logger::init();

    let db = DatabaseClient::new().await;

    db.test().await;

    let filters = all(Arc::new(db));

    warp::serve(filters).run(([127, 0, 0, 1], 3030)).await;
}
