use postgres::{Client, Row};

use openssl::ssl::{SslConnector, SslMethod};


use postgres_openssl::MakeTlsConnector;

use std::error;

fn main() -> Result<(), Box<dyn error::Error>> {
    let builder = SslConnector::builder(SslMethod::tls())?;

    let connector = MakeTlsConnector::new(builder.build());

    let mut client = Client::connect(
***REMOVED***
        connector,
    )?;

    let _tables = get_table_names(&mut client)?;

    Ok(())
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
