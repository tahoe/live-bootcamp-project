use dotenvy::dotenv;
use lazy_static::lazy_static;
use std::env as std_env;

// Define a lazily evaluated static. lazy_static is needed because std_env::var is not a const function.
lazy_static! {
    pub static ref JWT_SECRET: String = set_token();
    pub static ref JWT_COOKIE_DOMAIN: String = set_domain();
    pub static ref DATABASE_URL: String = set_db_url();
}

fn set_token() -> String {
    dotenv().ok(); // Load environment variables
    let secret = std_env::var("JWT_SECRET").expect("JWT_SECRET must be set.");
    if secret.is_empty() {
        panic!("JWT_SECRET must not be empty.");
    }
    secret
}

fn set_domain() -> String {
    dotenv().ok(); // Load environment variables
    let domain = std_env::var("JWT_COOKIE_DOMAIN").expect("JWT_COOKIE_DOMAIN must be set.");
    if domain.is_empty() {
        panic!("JWT_COOKIE_DOMAIN must not be empty.");
    }
    domain
}

fn set_db_url() -> String {
    dotenv().ok(); // Load environment variables
    let db_url = std_env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    if db_url.is_empty() {
        panic!("DATABASE_URL must not be empty.");
    }
    db_url
}

pub const JWT_COOKIE_NAME: &str = "jwt";

pub mod prod {
    pub const APP_ADDRESS: &str = "0.0.0.0:3000";
}

pub mod test {
    pub const APP_ADDRESS: &str = "127.0.0.1:0";
    pub const JWT_COOKIE_DOMAIN: &str = "127.0.0.1";
}
