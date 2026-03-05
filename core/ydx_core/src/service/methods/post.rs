use crate::service::client::HttpClient;
use anyhow::Result;

impl HttpClient {
    pub async fn signin(&self, email: String, password: String) -> Result<String> {
        self.post(
            "/api/signin",
            serde_json::json!({"email": email,"password": password}),
        )
        .await
    }

    pub async fn signup(&self, email: String, password: String) -> Result<String> {
        self.post(
            "/api/signup",
            serde_json::json!({"email": email,"password": password}),
        )
        .await
    }
}
