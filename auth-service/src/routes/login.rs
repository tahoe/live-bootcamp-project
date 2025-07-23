use axum::Json;
pub async fn login() -> Json<Vec<String>> {
    Json(vec!["log".to_owned(), "in".to_owned()])
}
