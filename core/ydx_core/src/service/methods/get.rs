use anyhow::Result;
use crate::service::client::HttpClient;

impl HttpClient {
    pub async fn get_signcheck(&self) -> Result<String> {
        self.get("/api/signcheck").await
    }
}
