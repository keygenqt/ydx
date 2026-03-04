use crate::service::client::HttpClient;
use anyhow::Result;

impl HttpClient {
    pub async fn signcheck(&self) -> Result<String> {
        self.get("/api/signcheck").await
    }
}
