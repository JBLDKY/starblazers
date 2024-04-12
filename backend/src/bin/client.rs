#![allow(dead_code)]
use postgres::{Client, Row};
use service::database::db::DatabaseClient;

fn main() {
    std::env::set_var("rust_log", "debug");
    pretty_env_logger::init();

    let mut db_client = DatabaseClient::new().unwrap();
    for row in db_client.head().unwrap() {
        let uname: &str = row.get("username");
        log::info!("Found uname: {}", uname);
    }
}

fn get_table_names(client: &mut Client) -> Result<Vec<Row>, postgres::Error> {
    client.query(
        "
    SELECT table_name
    FROM information_schema.tables
    WHERE table_type = 'BASE TABLE'
    AND table_schema = 'public';
        ",
        &[],
    )
}
