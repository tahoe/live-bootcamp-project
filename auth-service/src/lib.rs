use axum::routing::post;
use axum::{response::IntoResponse, serve::Serve, Json, Router};
use std::error::Error;
use tower_http::services::ServeDir;

// This struct encapsulates our application-related logic.
pub struct Application {
    server: Serve<Router, Router>,
    // address is exposed as a public field
    // so we have access to it in tests.
    pub address: String,
}

impl Application {
    pub async fn build(address: &str) -> Result<Application, Box<dyn Error>> {
        // Move the Router definition from 'main.rs' to here.
        // Also, remove the `hello` route,
        // We don't need it at this point!
        let router = Router::new()
            .nest_service("/", ServeDir::new("assets"))
            .route("/signup", post(signup))
            .route("/login", post(login))
            .route("/logout", post(logout))
            .route("/verify_2fa", post(verify_2fa))
            .route("/verify_token", post(verify_token));

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

async fn signup() -> Json<Vec<String>> {
    Json(vec!["sign".to_owned(), "up".to_owned()])
}

async fn login() -> impl IntoResponse {
    Json(vec!["log".to_owned(), "in".to_owned()])
}

async fn logout() -> impl IntoResponse {
    Json(vec!["log".to_owned(), "out".to_owned()])
}

async fn verify_2fa() -> impl IntoResponse {
    Json(vec!["verify".to_owned(), "2fa".to_owned()])
}

async fn verify_token() -> impl IntoResponse {
    Json(vec!["verify".to_owned(), "token".to_owned()])
}
