use crate::service::client::HttpClient;
use anyhow::Result;

impl HttpClient {
    pub async fn signout(&self) -> Result<String> {
        self.delete("/api/signout").await
    }
}
