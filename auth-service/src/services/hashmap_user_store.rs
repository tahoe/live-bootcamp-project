use std::collections::HashMap;

use crate::domain::User;

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

#[derive(Default)]
pub struct HashmapUserStore {
    users: HashMap<String, User>,
}

impl HashmapUserStore {
    pub fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        if self.users.contains_key(&user.email.to_owned()) {
            return Err(UserStoreError::UserAlreadyExists);
        }
        self.users.insert(user.email.to_owned(), user);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_user() {
        let user = User::new("test@test.com".to_owned(), "password".to_owned(), true);
        let users = HashMap::new();
        let mut mapper = HashmapUserStore { users };
        let _ = mapper.add_user(user);
    }

    #[tokio::test]
    async fn test_get_user() {
        // let user = User::new("test@test.com".to_owned(), "password".to_owned(), true);
        // let users = HashMap::new();
        // let mut mapper = HashmapUserStore { users };
        // let _ = mapper.add_user(user.clone());
        // assert_eq!(mapper.users.get("test@test.com"), Some(&user.clone()));
        todo!()
    }

    #[tokio::test]
    async fn test_validate_user() {
        todo!()
    }
}
