use crate::general::spawn_app;
use std::collections::HashMap;

#[actix_web::test]
async fn login_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let user = app
        .new_test_user()
        .await
        .expect("Failed to create test user");

    let mut map = HashMap::new();
    map.insert("username", user.username);
    map.insert("password", user.password);

    let response = client
        .post(&format!("{}/auth/login", &app.address))
        .json(&map)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());

    let auth_header = response.headers().get("authorization");
    assert!(auth_header.is_some());
    assert!(auth_header
        .unwrap()
        .to_str()
        .expect("could not parse the header to a string")
        .starts_with("Bearer "));
}

#[actix_web::test]
async fn login_returns_a_jwt() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let user = app
        .new_test_user()
        .await
        .expect("Failed to create test user");

    let mut map = HashMap::new();
    map.insert("username", user.username);
    map.insert("password", user.password);

    let response = client
        .post(&format!("{}/auth/login", &app.address))
        .json(&map)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
}
