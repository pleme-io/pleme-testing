//! # pleme-testing
//!
//! Testing utilities library for Pleme platform services.
//!
//! ## Features
//!
//! - **Test Fixtures** - Reusable test data builders
//! - **Testcontainers** - Docker containers for integration tests
//! - **Database Fixtures** - PostgreSQL and Redis test helpers
//! - **HTTP Mocking** - HTTP client testing utilities
//!
//! ## Usage
//!
//! ```rust
//! use pleme_testing::fixtures::UserFixture;
//!
//! #[tokio::test]
//! async fn test_user_creation() {
//!     let user = UserFixture::default().build();
//!     assert!(!user.id.is_empty());
//! }
//! ```

pub mod fixtures;

#[cfg(feature = "containers")]
pub mod containers;

#[cfg(feature = "database")]
pub mod database;

#[cfg(feature = "http")]
pub mod http;

pub use fixtures::Fixture;

use thiserror::Error;

/// Testing errors
#[derive(Error, Debug)]
pub enum TestingError {
    #[error("Fixture creation failed: {0}")]
    FixtureFailed(String),

    #[error("Container startup failed: {0}")]
    ContainerFailed(String),

    #[error("Test setup failed: {0}")]
    SetupFailed(String),
}

/// Result type for testing operations
pub type Result<T> = std::result::Result<T, TestingError>;
