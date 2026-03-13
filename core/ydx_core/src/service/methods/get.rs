use crate::service::client::HttpClient;
use anyhow::Result;

impl HttpClient {
    pub async fn signcheck(&self, token: Option<String>) -> Result<String> {
        self.get("/api/signcheck", token).await
    }

    pub async fn get_user(&self, id: String, token: Option<String>) -> Result<String> {
        self.get(&format!("/api/users/{id}"), token).await
    }
}
