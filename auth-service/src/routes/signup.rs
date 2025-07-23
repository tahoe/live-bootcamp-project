use axum::Json;
pub async fn signup() -> Json<Vec<String>> {
    Json(vec!["sign".to_owned(), "up".to_owned()])
}
