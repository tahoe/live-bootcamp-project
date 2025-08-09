use crate::helpers::TestApp;
use auth_service::domain::Email;
use auth_service::utils::auth::generate_auth_token;

#[tokio::test]
async fn verify_token_returns_ok() {
    let app = TestApp::new().await;

    let email = Email::parse("test@example.com".to_owned()).unwrap();
    let token = generate_auth_token(&email).unwrap();
    let response = app.post_verify_token(&token).await;

    assert_eq!(response.status().as_u16(), 200);
}
