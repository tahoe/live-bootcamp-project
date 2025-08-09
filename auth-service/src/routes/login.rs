use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use serde::Deserialize;

use crate::{
    app_state::AppState,
    domain::{AuthAPIError, Email, Password},
    utils::auth::generate_auth_cookie,
};

pub async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(request): Json<LoginRequest>,
) -> Result<(CookieJar, impl IntoResponse), AuthAPIError> {
    let password = match Password::parse(request.password) {
        Ok(password) => password,
        Err(_) => return Err(AuthAPIError::InvalidCredentials),
    };

    let email = match Email::parse(request.email) {
        Ok(email) => email,
        Err(_) => return Err(AuthAPIError::InvalidCredentials),
    };

    let user_store = &state.user_store.read().await;

    if user_store.validate_user(&email, &password).await.is_err() {
        return Err(AuthAPIError::IncorrectCredentials);
    }

    let user = match user_store.get_user(&email).await {
        Ok(user) => user,
        Err(_) => return Err(AuthAPIError::IncorrectCredentials),
    };

    // last check, make sure their passwords match
    user_store
        .validate_user(&user.email, &user.password)
        .await
        .map_err(|_| AuthAPIError::IncorrectCredentials)?;

    let auth_cookie = match generate_auth_cookie(&user.email) {
        Ok(cookie) => cookie,
        Err(_) => return Err(AuthAPIError::UnexpectedError),
    };

    let updated_jar = jar.add(auth_cookie);

    Ok((updated_jar, StatusCode::OK.into_response()))
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
    // #[serde(rename = "requires2FA")]
    // pub requires_2fa: bool,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct LoginResponse {
    pub email: String,
    pub password: String,
}
