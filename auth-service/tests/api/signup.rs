use crate::helpers::{get_random_email, TestApp};

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    // TODO: add more malformed input test cases
    let test_cases = [serde_json::json!({
        "password": "pass123",
        "requires2FA": true
    })];

    for test_case in test_cases.iter() {
        let response = app.post_signup(&random_email).await;
        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {test_case}"
        );
    }
}
