//! Test fixtures and builders

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Fixture trait for test data builders
pub trait Fixture: Sized {
    type Output;

    /// Build the fixture
    fn build(self) -> Self::Output;

    /// Build with custom modifications
    fn build_with<F>(self, f: F) -> Self::Output
    where
        F: FnOnce(&mut Self::Output),
    {
        let mut output = self.build();
        f(&mut output);
        output
    }
}

/// Example user fixture
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub name: String,
}

/// User fixture builder
#[derive(Default)]
pub struct UserFixture {
    id: Option<String>,
    email: Option<String>,
    name: Option<String>,
}

impl UserFixture {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
}

impl Fixture for UserFixture {
    type Output = User;

    fn build(self) -> User {
        User {
            id: self.id.unwrap_or_else(|| Uuid::new_v4().to_string()),
            email: self.email.unwrap_or_else(|| "test@example.com".to_string()),
            name: self.name.unwrap_or_else(|| "Test User".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_fixture() {
        let user = UserFixture::default().build();
        assert!(!user.id.is_empty());
        assert_eq!(user.email, "test@example.com");
    }

    #[test]
    fn test_user_fixture_custom() {
        let user = UserFixture::new()
            .email("custom@example.com")
            .name("Custom User")
            .build();
        assert_eq!(user.email, "custom@example.com");
        assert_eq!(user.name, "Custom User");
    }
}
