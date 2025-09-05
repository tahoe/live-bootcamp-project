use super::Email;
use color_eyre::eyre::Result;

// All email clients should implement this interface
#[async_trait::async_trait]
pub trait EmailClient {
    async fn send_email(&self, recipient: &Email, subject: &str, content: &str) -> Result<()>;
}
