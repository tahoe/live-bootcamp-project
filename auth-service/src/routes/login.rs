use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState,
    domain::{AuthAPIError, Email, Password, User, UserStore},
};

pub async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let email =
        Email::parse(request.email.clone()).map_err(|_| AuthAPIError::InvalidCredentials)?;
    let password =
        Password::parse(request.password.clone()).map_err(|_| AuthAPIError::InvalidCredentials)?;

    let user = User::new(email, password, request.requires_2fa);

    let user_store = state.user_store.read().await;

    // last check, make sure their passwords match
    user_store
        .validate_user(&user.email, &user.password)
        .await
        .map_err(|_| AuthAPIError::IncorrectCredentials)?;

    let response = Json(LoginResponse {
        message: "User logged in successfully!".to_string(),
    });

    Ok((StatusCode::ACCEPTED, response))
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct LoginResponse {
    pub message: String,
}
