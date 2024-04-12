use crate::cli::commands::executable::Executable;
use clap::Parser;

use crate::database::db::DatabaseClient;

#[derive(Parser, Debug)]
pub struct CreatePlayer {
    #[clap(long)]
    email: String,
    #[clap(long)]
    username: String,
    #[clap(long)]
    password: String,
    #[clap(long, default_value_t = 0)]
    games_played: i32,
}

impl Executable for CreatePlayer {
    fn execute(&self, db: &mut DatabaseClient) {
        match db.execute_query(
            self.query(),
            &[
                &self.email,
                &self.username,
                &self.password,
                &self.games_played,
            ],
        ) {
            Ok(v) => log::info!("Succesfully created a new record: {:?}", v),
            Err(e) => log::error!(
                "Could not create record because of the following error: {}",
                e
            ),
        };
    }
    fn query(&self) -> &str {
        r#"
    INSERT INTO players (email, username, password, games_played)
    VALUES ($1, $2, $3, $4)
    RETURNING id;
"#
    }
}
