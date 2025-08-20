use app_state::AppState;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    serve::Serve,
    Json, Router,
};
use domain::AuthAPIError;
use serde::{Deserialize, Serialize};
use std::error::Error;
use tower_http::{cors::CorsLayer, services::ServeDir};

pub mod app_state;
pub mod domain;
pub mod routes;
pub mod services;
pub mod utils;

// This struct encapsulates our application-related logic.
pub struct Application {
    server: Serve<Router, Router>,
    // address is exposed as a public field
    // so we have access to it in tests.
    pub address: String,
}

impl Application {
    pub async fn build(app_state: AppState, address: &str) -> Result<Self, Box<dyn Error>> {
        // Move the Router definition from 'main.rs' to here.
        // Also, remove the `hello` route,
        // We don't need it at this point!
        // let allowed_origins = [
        //     "http://localhost:8000".parse()?,
        //     "http://auth-api.durling.net".parse()?,
        //     "http://auth-app.durling.net".parse()?,
        // ];

        // trying out very_permissive instead of new function
        let cors = CorsLayer::very_permissive();
        // let cors = CorsLayer::new()
        //     .allow_methods([Method::GET, Method::POST])
        //     .allow_credentials(true)
        //     .allow_origin(allowed_origins);

        let router = Router::new()
            .nest_service("/", ServeDir::new("assets"))
            .route("/signup", post(routes::signup))
            .route("/login", post(routes::login))
            .route("/logout", post(routes::logout))
            .route("/verify-2fa", post(routes::verify_2fa))
            .route("/verify-token", post(routes::verify_token))
            .with_state(app_state)
            .layer(cors);

        let listner = tokio::net::TcpListener::bind(address).await?;
        let address = listner.local_addr()?.to_string();
        let server = axum::serve(listner, router);

        // Create a new Application instance and return it.
        Ok(Self { server, address })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on {}", &self.address);
        self.server.await
    }
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl IntoResponse for AuthAPIError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthAPIError::UserAlreadyExists => (StatusCode::CONFLICT, "User already exists"),
            AuthAPIError::InvalidCredentials => (StatusCode::BAD_REQUEST, "Invalid credentials"),
            AuthAPIError::IncorrectCredentials => (StatusCode::UNAUTHORIZED, "Non Shall Pass!"),
            AuthAPIError::MissingToken => (StatusCode::BAD_REQUEST, "Please provide a tolken!"),
            AuthAPIError::InvalidToken => (StatusCode::UNAUTHORIZED, "Tolken invalid!"),
            AuthAPIError::UnexpectedError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Unexpected error")
            }
        };
        let body = Json(ErrorResponse {
            error: error_message.to_string(),
        });
        (status, body).into_response()
    }
}
