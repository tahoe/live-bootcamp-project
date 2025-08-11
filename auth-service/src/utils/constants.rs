use dotenvy::dotenv;
use lazy_static::lazy_static;
use std::env as std_env;

// Define a lazily evaluated static. lazy_static is needed because std_env::var is not a const function.
lazy_static! {
    pub static ref JWT_SECRET: String = set_token();
}

fn set_token() -> String {
    dotenv().ok(); // Load environment variables
    let secret = std_env::var("JWT_SECRET").expect("JWT_SECRET must be set.");
    if secret.is_empty() {
        panic!("JWT_SECRET must not be empty.");
    }
    secret
}

lazy_static! {
    pub static ref JWT_COOKIE_DOMAIN: String = set_domain();
}

fn set_domain() -> String {
    dotenv().ok(); // Load environment variables
    let domain = std_env::var("JWT_COOKIE_DOMAIN").expect("JWT_COOKIE_DOMAIN must be set.");
    if domain.is_empty() {
        panic!("JWT_COOKIE_DOMAIN must not be empty.");
    }
    domain
}

pub const JWT_COOKIE_NAME: &str = "jwt";

pub mod prod {
    pub const APP_ADDRESS: &str = "0.0.0.0:3000";
}

pub mod test {
    pub const APP_ADDRESS: &str = "127.0.0.1:0";
}
