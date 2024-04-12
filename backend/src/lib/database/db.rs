use super::queries::{PlayerEntry, PlayerField, Table};
use anyhow::{Context, Result}; // Make sure to import Context for using `.context()`
use dotenv::dotenv;
use openssl::ssl::{SslConnector, SslMethod};
use postgres::{types::ToSql, Client, Error, Row};
use postgres_openssl::MakeTlsConnector;
use rand::Rng;
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
        let db_url = env::var("SQL_DB").expect("Missing environment variable: `SQL_DB`");
        let client = Client::connect(&db_url, connector)?;
        Ok(DatabaseClient { client })
    }

    pub fn reset_table(&mut self, table: &Table) -> Result<Vec<Row>, Error> {
        let sql = table.drop();
        self.execute_query(sql, &[])
    }

    pub fn add_record(&mut self, username: &str) {
        let password = DatabaseClient::generate_password();
        let email = DatabaseClient::generate_email(username);
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
        const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
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

    /// Create a new table with the provided name.
    pub fn create_table(&mut self, name: &Table) -> Result<(), Error> {
        let create_table_sql = format!(
            "CREATE TABLE IF NOT EXISTS {} (
            id SERIAL PRIMARY KEY,
            data TEXT NOT NULL
        );",
            name
        );
        self.client.execute(&create_table_sql, &[])?;
        Ok(())
    }

    /// Wipes the entire database.
    pub fn wipe(&mut self) -> Result<(), Error> {
        let tables = self.client.query(
            "SELECT table_name FROM information_schema.tables WHERE table_schema = 'public'",
            &[],
        )?;
        for row in tables {
            let table_name: &str = row.get(0);
            let drop_command = format!("DROP TABLE IF EXISTS {}", table_name);
            self.client.execute(&drop_command, &[])?;
        }
        Ok(())
    }

    pub async fn update_player_field(
        &mut self,
        player: &PlayerEntry,
        field: &PlayerField,
        value: &str,
    ) -> Result<()> {
        let (sql, params) = player
            .update_field(field, value)
            .map_err(|e| anyhow::anyhow!("Failed to create PlayerField update query: {}", e))?;

        self.client
            .execute(
                &sql,
                &params
                    .iter()
                    .map(|p| p.as_ref() as &(dyn ToSql + Sync))
                    .collect::<Vec<_>>(),
            )
            .context("Database execution failed")?;

        Ok(())
    }
}
