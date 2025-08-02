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
    pub fn new() -> Self {
        let users = HashMap::new();
        Self { users }
    }

    pub fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        if self.users.contains_key(&user.email.to_owned()) {
            return Err(UserStoreError::UserAlreadyExists);
        }
        self.users.insert(user.email.to_owned(), user);
        Ok(())
    }

    pub fn get_user(&mut self, email: &str) -> Result<User, UserStoreError> {
        let user_ret = self.users.get(email);
        if let Some(user) = user_ret {
            Ok(user.clone())
        } else {
            Err(UserStoreError::UserNotFound)
        }
    }

    pub fn validate_user(&mut self, email: &str, password: &str) -> Result<(), UserStoreError> {
        let user_ret = self.users.get(email);
        if let Some(user) = user_ret {
            if user.password == password {
                Ok(())
            } else {
                Err(UserStoreError::InvalidCredentials)
            }
        } else {
            Err(UserStoreError::UserNotFound)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_user() {
        let user = User::new("test@test.com".to_owned(), "password".to_owned(), true);
        let mut mapper = HashmapUserStore::new();
        let _ = mapper.add_user(user);
    }

    #[tokio::test]
    async fn test_get_user() {
        let user = User::new("test@test.com".to_owned(), "password".to_owned(), true);
        let mut mapper = HashmapUserStore::new();
        let _ = mapper.add_user(user.clone());
        assert_eq!(mapper.get_user("test@test.com"), Ok(user.clone()));
    }

    #[tokio::test]
    async fn test_validate_user() {
        let user = User::new("test@test.com".to_owned(), "password".to_owned(), true);
        let mut mapper = HashmapUserStore::new();
        let _ = mapper.add_user(user.clone());
        assert_eq!(mapper.validate_user("test@test.com", "password"), Ok(()));
    }
}
