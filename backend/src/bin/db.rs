pub mod db {
    use openssl::ssl::{SslConnector, SslMethod};

    use postgres_openssl::MakeTlsConnector;

    use rand::Rng;

    use dotenv::dotenv;

    use postgres::{types::ToSql, Client, Error, Row};
    use std::env;

    pub struct DatabaseClient {
        client: Client,
    }

    impl DatabaseClient {
        // constructor
        pub fn new() -> Result<Self, Error> {
            dotenv().ok();
            let builder = SslConnector::builder(SslMethod::tls()).expect("IDK man");
            let connector = MakeTlsConnector::new(builder.build());
            let db_url = env::var("SQL_DB").unwrap();
            let client = Client::connect(&db_url, connector)?;
            Ok(DatabaseClient { client })
        }

        pub fn reset_table(&mut self, table_name: &str) {
            let _ = self.execute_query("DELETE FROM $1", &[&table_name]);
            println!("Succesfully cleared table: {}", table_name);
        }

        pub fn add_record(&mut self, username: &str) {
            let password = DatabaseClient::generate_password();
            let email = DatabaseClient::generate_email(&username);
            let _ = self.execute_query(
                "INSERT INTO users (email, username, password) VALUES
                ($1, $2, $3)",
                &[&email, &username, &password],
            );
        }

        pub fn head(&mut self) -> Result<Vec<Row>, postgres::Error> {
            self.execute_query("SELECT * FROM users LIMIT 10;", &[])
        }

        // random password generator, not good obviously..
        pub fn generate_password() -> String {
            const CHARSET: &[u8] =
                b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
            let mut rng = rand::thread_rng();
            let password: String = (0..15)
                .map(|_| {
                    let idx = rng.gen_range(0..CHARSET.len());
                    CHARSET[idx] as char
                })
                .collect();
            password
        }

        pub fn generate_email(username: &str) -> String {
            format!("{username}@email.za")
        }

        pub fn execute_query(
            &mut self,
            query: &str,
            params: &[&(dyn ToSql + Sync)],
        ) -> Result<Vec<Row>, Error> {
            self.client.query(query, params)
        }
    }
}

/*fn main() -> Result<(), Box<dyn error::Error>> {
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
}*/
