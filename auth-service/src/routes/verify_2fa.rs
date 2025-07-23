use axum::Json;
pub async fn verify_2fa() -> Json<Vec<String>> {
    Json(vec!["verify".to_owned(), "2fa".to_owned()])
}
