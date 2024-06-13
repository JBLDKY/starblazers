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

    assert!(response.status().is_success());
}

#[tokio::test]
async fn signup_does_not_work_if_a_field_is_empty() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("", "my_funni_password", "my_funni@mail.com", "No username"), // TODO: Currently this is allowed
        ("my_funni_username", "", "my_funni@mail.com", "No password"),
        ("my_funni_username", "my_funni_password", "", "No email"),
    ];

    for case in test_cases {
        let mut map = HashMap::new();
        map.insert("username", case.0);
        map.insert("password", case.1);
        map.insert("email", case.2);

        let response = client
            .post(&format!("{}/auth/signup", &app.address))
            .json(&map)
            .send()
            .await
            .expect("Failed to execute request");

        assert_eq!(
            response.status().as_u16(),
            400,
            "The api did not return a 400 when request was sent that had `{}`",
            case.3
        );
    }
}
