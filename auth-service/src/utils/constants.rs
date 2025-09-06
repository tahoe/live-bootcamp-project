use dotenvy::dotenv;
use lazy_static::lazy_static;
use secrecy::Secret;
use std::env as std_env;

// Define a lazily evaluated static. lazy_static is needed because std_env::var is not a const function.
lazy_static! {
    pub static ref JWT_SECRET: Secret<String> = set_token();
    pub static ref DATABASE_URL: Secret<String> = set_db_url();
    pub static ref POSTMARK_AUTH_TOKEN: Secret<String> = set_postmark_auth_token();
    pub static ref REDIS_HOST_NAME: String = set_redis_host();
    pub static ref JWT_COOKIE_DOMAIN: String = set_domain();
}

fn set_token() -> Secret<String> {
    dotenv().ok(); // Load environment variables
    let secret = std_env::var("JWT_SECRET").expect("JWT_SECRET must be set.");
    if secret.is_empty() {
        panic!("JWT_SECRET must not be empty.");
    }
    Secret::new(secret)
}

fn set_db_url() -> Secret<String> {
    dotenv().ok(); // Load environment variables
    let db_url = std_env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    if db_url.is_empty() {
        panic!("DATABASE_URL must not be empty.");
    }
    Secret::new(db_url)
}

fn set_postmark_auth_token() -> Secret<String> {
    dotenv().ok(); // Load environment variables
    let auth_token = std_env::var("POSTMARK_AUTH_TOKEN").expect("POSTMARK_AUTH_TOKEN must be set.");
    if auth_token.is_empty() {
        panic!("POSTMARK_AUTH_TOKEN must not be empty.");
    }
    Secret::new(auth_token)
}

fn set_domain() -> String {
    dotenv().ok(); // Load environment variables
    let domain = std_env::var("JWT_COOKIE_DOMAIN").expect("JWT_COOKIE_DOMAIN must be set.");
    if domain.is_empty() {
        panic!("JWT_COOKIE_DOMAIN must not be empty.");
    }
    domain
}

fn set_redis_host() -> String {
    dotenv().ok();
    let redis_host = std_env::var("REDIS_HOST_NAME").expect("REDIS_HOST_NAME must be set.");
    if redis_host.is_empty() {
        panic!("REDIS_HOST_NAME must not be empty.");
    }
    redis_host
}

pub const JWT_COOKIE_NAME: &str = "jwt";
pub const DEFAULT_REDIS_HOSTNAME: &str = "127.0.0.1";

pub mod prod {
    pub const APP_ADDRESS: &str = "0.0.0.0:3000";

    pub mod email_client {
        use std::time::Duration;

        pub const BASE_URL: &str = "https://api.postmarkapp.com/email";
        pub const SENDER: &str = "djdtahoe@gmail.com";
        pub const TIMEOUT: Duration = std::time::Duration::from_millis(1000);
    }
}

pub mod test {
    pub const APP_ADDRESS: &str = "127.0.0.1:0";
    pub const JWT_COOKIE_DOMAIN: &str = "127.0.0.1";

    pub mod email_client {
        use std::time::Duration;
        pub const SENDER: &str = "test@email.com";
        pub const TIMEOUT: Duration = std::time::Duration::from_millis(2000);
    }
}
