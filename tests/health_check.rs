use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let addrs = spawn_app();
    // We need to bring in `reqwest`
    // to perform HTTP requests against our application.
    let client = reqwest::Client::new();
    // Act
    let response = client
        .get(addrs + "/health_check")
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
// Launch our application in the background ~somehow~
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();

    let server = actix_api::run(listener).expect("Failed to bind address");
    // Launch the server as a background task
    tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}