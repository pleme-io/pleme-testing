//! Testcontainers integration

use crate::Result;

/// PostgreSQL testcontainer helper
pub struct PostgresContainer;

impl PostgresContainer {
    /// Start PostgreSQL container
    pub async fn start() -> Result<String> {
        // Placeholder - would use testcontainers crate
        Ok("postgres://localhost:5432/test".to_string())
    }
}

/// Redis testcontainer helper
pub struct RedisContainer;

impl RedisContainer {
    /// Start Redis container
    pub async fn start() -> Result<String> {
        // Placeholder - would use testcontainers crate
        Ok("redis://localhost:6379".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_postgres_container() {
        let url = PostgresContainer::start().await.unwrap();
        assert!(url.starts_with("postgres://"));
    }

    #[tokio::test]
    async fn test_redis_container() {
        let url = RedisContainer::start().await.unwrap();
        assert!(url.starts_with("redis://"));
    }
}
