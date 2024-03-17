use postgres::Client;

use openssl::ssl::{SslConnector, SslMethod};

use postgres_openssl::MakeTlsConnector;

use std::error;

use dotenv::dotenv;

use std::env;

fn main() -> Result<(), Box<dyn error::Error>> {
    // load .env file contents
    dotenv().ok();

    let builder = SslConnector::builder(SslMethod::tls())?;

    let connector = MakeTlsConnector::new(builder.build());

    let db_url = env::var("SQL_DB").unwrap();

    let mut client = Client::connect(&db_url, connector)?;

    /*for row in client.query("SELECT 42", &[])? {
            let ret: i32 = row.get(0);

            println!("Result = {}", ret);
        }
        client.batch_execute(
            "
        CREATE TABLE IF NOT EXISTS jordgay (
            id      SERIAL PRIMARY KEY,
            name    TEXT NOT NULL,
            data    BYTEA
        )
    ",
        )?;*/

    let result = client.query(
        "
    SELECT table_name
    FROM information_schema.tables
    WHERE table_type = 'BASE TABLE'
    AND table_schema = 'public';
        ",
        &[],
    )?;

    for row in &result {
        let thing: String = row.get("table_name");
        println!("{}", thing);
    }

    println!("{:#?}", result);

    Ok(())
}
