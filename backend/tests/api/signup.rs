use actix_web::http::StatusCode;

use crate::general::spawn_app;
use std::collections::HashMap;

#[tokio::test]
async fn signup_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let mut map = HashMap::new();
    map.insert("username", "my_funni_username");
    map.insert("password", "my_funni_password");
    map.insert("email", "my_funni@mail.com");

    let response = client
        .post(&format!("{}/auth/signup", &app.address))
        .json(&map)
        .send()
        .await
        .expect("Failed to execute request");

    dbg!(&response);
    assert!(response.status().is_success());
}
