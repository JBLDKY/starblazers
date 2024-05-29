#![allow(dead_code)]
use postgres::{Client, Row};
// use service::cli::handle_cli_input;

fn main() {
    std::env::set_var("RUST_LOG", "info");
    pretty_env_logger::init();
    log::info!("Starting starburst client");

    // handle_cli_input();
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
