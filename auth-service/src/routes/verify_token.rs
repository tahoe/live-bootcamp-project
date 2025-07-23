use axum::Json;
pub async fn verify_token() -> Json<Vec<String>> {
    Json(vec!["verify".to_owned(), "token".to_owned()])
}
