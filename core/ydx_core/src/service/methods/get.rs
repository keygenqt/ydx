use crate::service::client::HttpClient;
use anyhow::Result;

impl HttpClient {
    pub async fn signcheck(&self, token: Option<String>) -> Result<String> {
        self.get("/api/signcheck", token).await
    }
}
