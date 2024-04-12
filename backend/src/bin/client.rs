use postgres::{Client, Row};

use openssl::ssl::{SslConnector, SslMethod};

use postgres_openssl::MakeTlsConnector;

use std::error;

pub mod db;
use crate::db::db::DatabaseClient;

fn main() {
    /*let builder = SslConnector::builder(SslMethod::tls())?;

    let connector = MakeTlsConnector::new(builder.build());

    let mut client = Client::connect(
        "***REMOVED***",
        connector,
    )?;

    let _tables = get_table_names(&mut client)?;

    Ok(())*/

    let mut db_client = DatabaseClient::new().unwrap();
    for row in db_client.head().unwrap() {
        let uname: &str = row.get("username");
        println!("{}", uname);
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
