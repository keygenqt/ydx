use crate::service::client::HttpClient;
use anyhow::Result;

impl HttpClient {
    pub async fn signout(&self, token: Option<String>) -> Result<String> {
        self.delete("/api/signout", token).await
    }
}
