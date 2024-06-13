use crate::general::spawn_app;
use service::{claims::Claims, types::LoginMethod};

#[tokio::test]
async fn verify_jwt_works() {
    let app = spawn_app().await;

    let user = app
        .new_test_user()
        .await
        .expect("failed to create test user");

    let user_record = app
        .db_client
        .get_details_by_login_method(&LoginMethod::Email(user.email))
        .await
        .expect("Failed to retrieve user record");

    let jwt = Claims::generate_jwt(user_record).expect("Could not create jwt");

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/auth/verify_jwt", &app.address))
        .header("authorization", jwt)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
}
