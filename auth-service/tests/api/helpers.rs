#![allow(unused)]
use auth_service::domain::{Email, Password, User, UserStore};
use auth_service::{app_state::AppState, services::HashmapUserStore, Application};
use reqwest::Client;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub http_client: reqwest::Client,
}

impl TestApp {
    pub async fn new() -> Self {
        // Create a pre-defined test user for auth/login tests
        let mut mapper = HashmapUserStore::default();
        let user = User {
            email: Email::parse("test@example.com".to_owned()).unwrap(),
            password: Password::parse("Secure Password".to_owned()).unwrap(),
            requires_2fa: false,
        };

        // Add the user to the hashmap/user store before launching test app
        let _result = mapper.add_user(user.clone()).await;

        // Add the hashmap to the app
        let mut user_store = Arc::new(RwLock::new(mapper));
        let app_state = AppState::new(user_store);
        let app = Application::build(app_state, "0.0.0.0:0")
            .await
            .expect("Failed to build app");

        // for return only, hsa to be before spawning the app and cloned...
        let address = format!("http://{}", app.address.clone());

        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());

        let http_client = Client::new();

        Self {
            address,
            http_client,
        }
    }

    pub async fn get_root(&self) -> reqwest::Response {
        self.http_client
            .get(format!("{}/", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_signup<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(format!("{}/signup", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_login<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(format!("{}/login", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_logout(&self) -> reqwest::Response {
        self.http_client
            .post(format!("{}/logout", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_verify_2fa(&self) -> reqwest::Response {
        self.http_client
            .post(format!("{}/verify_2fa", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_verify_token(&self) -> reqwest::Response {
        self.http_client
            .post(format!("{}/verify_token", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }
}

pub fn get_random_email() -> String {
    format!("{}@example.com", Uuid::new_v4())
}
