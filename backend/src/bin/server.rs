use service::application::Application;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    if dotenv::dotenv().is_err() {
        log::error!("Warning: Did not find .env file in current working directory!");
    }
    std::env::set_var("RUST_LOG", "debug");
    pretty_env_logger::init();

    let app = Application::build("127.0.0.1", "3030").await?;
    app.start().await?;

    Ok(())
}
