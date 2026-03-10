use crate::service::client::HttpClient;
use anyhow::Result;

impl HttpClient {
    pub async fn signin(&self, email: String, password: String) -> Result<String> {
        self.post(
            "/api/signin",
            None,
            serde_json::json!({"email": email,"password": password}),
        )
        .await
    }

    pub async fn signup(&self, email: String, password: String) -> Result<String> {
        self.post(
            "/api/signup",
            None,
            serde_json::json!({"email": email,"password": password}),
        )
        .await
    }
}
