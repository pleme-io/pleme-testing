//! HTTP testing utilities

use serde_json::Value;

/// HTTP test client wrapper
pub struct TestClient {
    base_url: String,
}

impl TestClient {
    /// Create new test client
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
        }
    }

    /// Make GET request
    pub async fn get(&self, path: &str) -> Result<Value, Box<dyn std::error::Error>> {
        #[cfg(feature = "http")]
        {
            let url = format!("{}{}", self.base_url, path);
            let response = reqwest::get(&url).await?;
            let json = response.json().await?;
            Ok(json)
        }

        #[cfg(not(feature = "http"))]
        {
            let _ = path;
            Ok(Value::Null)
        }
    }

    /// Make POST request
    pub async fn post(&self, path: &str, body: Value) -> Result<Value, Box<dyn std::error::Error>> {
        #[cfg(feature = "http")]
        {
            let url = format!("{}{}", self.base_url, path);
            let client = reqwest::Client::new();
            let response = client.post(&url).json(&body).send().await?;
            let json = response.json().await?;
            Ok(json)
        }

        #[cfg(not(feature = "http"))]
        {
            let _ = (path, body);
            Ok(Value::Null)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = TestClient::new("http://localhost:8080");
        assert_eq!(client.base_url, "http://localhost:8080");
    }
}
