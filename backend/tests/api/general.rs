use service::types::User;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;

use service::application::Application;
use service::configuration::{get_settings, Settings};

use service::database::db::DatabaseClient;

pub struct TestApp {
    pub address: String,
    pub db_client: DatabaseClient,
}

impl TestApp {
    pub async fn new_test_user(&self) -> Result<User, sqlx::Error> {
        let user = User {
            id: None,
            email: "test@test.com".to_string(),
            username: "test".to_string(),
            password: "test".to_string(),
            authority: None,
            creation_date: None,
            uuid: None,
        };

        self.db_client.create_user(&user).await?;

        Ok(user)
    }
}

pub async fn spawn_app() -> TestApp {
    let settings = {
        let mut c = get_settings().expect("Failed to get settings");
        c.database.database_name = Uuid::new_v4().to_string();
        c.application.port = 0;
        c
    };

    new_test_database(&settings).await;

    let app = Application::build(settings.clone())
        .await
        .expect("Failed to build application");

    let address = format!("http://127.0.0.1:{}", app.port());

    tokio::spawn(app.start());

    std::env::set_var("JWT_SECRET", "my-256-bit-secret");

    TestApp {
        address,
        db_client: DatabaseClient {
            pool: PgPoolOptions::new().connect_lazy_with(settings.database.with_db()),
        },
    }
}

async fn new_test_database(settings: &Settings) -> PgPool {
    let mut connection = PgConnection::connect_with(&settings.database.without_db())
        .await
        .expect("Failed to connect to Postgres");

    connection
        .execute(format!(r#"CREATE DATABASE "{}"; "#, settings.database.database_name).as_str())
        .await
        .expect("Failed to create database");

    let connection_pool = PgPool::connect_with(settings.database.with_db())
        .await
        .expect("Failed to connect to Postgres");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    std::env::set_var("DATABASE_URL", settings.database.connection_string());

    connection_pool
}
