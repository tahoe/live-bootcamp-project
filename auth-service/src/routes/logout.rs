use axum::Json;
pub async fn logout() -> Json<Vec<String>> {
    Json(vec!["log".to_owned(), "out".to_owned()])
}
