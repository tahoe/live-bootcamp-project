#![allow(unused)]
use auth_service::Application;
use reqwest::Client;
use std::collections::HashMap;

pub struct TestApp {
    pub address: String,
    pub http_client: reqwest::Client,
}

impl TestApp {
    pub async fn new() -> Self {
        let app = Application::build("127.0.0.1:0")
            .await
            .expect("Failed to build app");

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

    pub async fn post_signup(&self) -> reqwest::Response {
        let mut map = HashMap::new();
        map.insert("email", "test@email.com");
        map.insert("password", "password");
        map.insert("requires2FA", "duh, need 2fa");
        self.http_client
            .post(format!("{}/signup", &self.address))
            .json(&map)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_login(&self) -> reqwest::Response {
        let mut map = HashMap::new();
        map.insert("email", "test@email.com");
        map.insert("password", "password");
        map.insert("requires2FA", "duh, need 2fa");
        self.http_client
            .post(format!("{}/login", &self.address))
            .json(&map)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_logout(&self) -> reqwest::Response {
        let mut map = HashMap::new();
        map.insert("email", "test@email.com");
        map.insert("password", "password");
        map.insert("requires2FA", "duh, need 2fa");
        self.http_client
            .post(format!("{}/logout", &self.address))
            .json(&map)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_verify_2fa(&self) -> reqwest::Response {
        let mut map = HashMap::new();
        map.insert("email", "test@email.com");
        map.insert("password", "password");
        map.insert("requires2FA", "duh, need 2fa");
        self.http_client
            .post(format!("{}/verify_2fa", &self.address))
            .json(&map)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_verify_token(&self) -> reqwest::Response {
        let mut map = HashMap::new();
        map.insert("email", "test@email.com");
        map.insert("password", "password");
        map.insert("requires2FA", "duh, need 2fa");
        self.http_client
            .post(format!("{}/verify_token", &self.address))
            .json(&map)
            .send()
            .await
            .expect("Failed to execute request.")
    }
}
