use crate::helpers::{get_random_email, TestApp};

#[tokio::test]
async fn should_return_422_if_malformed_credentials() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    // for test_case in test_cases.iter() {
    let response = app.post_login(&random_email).await;
    assert_eq!(
        response.status().as_u16(),
        422,
        "Failed for input: {random_email}"
    );
    // }
}

#[tokio::test]
async fn should_return_400_if_invalid_credentials() {
    let app = TestApp::new().await;

    let test_cases = [serde_json::json!({
        "email":"",
        "password":"ecure Password",
        "requires2FA": true
    })];

    for test_case in test_cases.iter() {
        let response = app.post_login(test_case).await;
        assert_eq!(
            response.status().as_u16(),
            400,
            "Failed for input: {test_case}"
        );
    }
}

#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
    let app = TestApp::new().await;

    let test_cases = [serde_json::json!({
        "email":"test@example.com",
        "password":"ecure Password",
        "requires2FA": true
    })];

    for test_case in test_cases.iter() {
        let response = app.post_login(test_case).await;
        assert_eq!(
            response.status().as_u16(),
            401,
            "Failed for input: {test_case}"
        );
    }
}
