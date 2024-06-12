use service::application::Application;
use service::configuration::get_settings;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    if dotenv::dotenv().is_err() {
        log::error!("Warning: Did not find .env file in current working directory!");
    }
    std::env::set_var("RUST_LOG", "debug");
    pretty_env_logger::init();

    let settings = get_settings();
    dbg!(&settings);

    let app = Application::build(settings.expect("Failed to get settings file")).await?;
    app.start().await?;

    Ok(())
}
