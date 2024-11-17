//! health_check.rs

use std::net::TcpListener;

use zero2prod::run;

#[tokio::test]
async fn health_check_works() {
    let listener =
        TcpListener::bind("127.0.0.1:0").expect("Failed to bind listener to random port.");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Failed to spawn our app.");
    tokio::spawn(server);

    let client = reqwest::Client::new();

    let response = client
        .get(format!("http://127.0.0.1:{}/health_check", port))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
