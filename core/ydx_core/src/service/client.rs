use anyhow::{Result, anyhow};

pub struct HttpClient {
    uuid: String
}

impl HttpClient {
    pub fn new(uuid: String) -> Self {
        Self {
            uuid
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl HttpClient {
    pub async fn get(&self, url: &str) -> Result<String> {
    let response = reqwest::Client::new()
        .get(url)
        .header("X-Device-UUID", &self.uuid)
        .send()
        .await
        .map_err(|e| anyhow!("Request failed: {}", e))?;
    Ok(response.text()
        .await
        .map_err(|e| anyhow!("Failed to get text: {}", e))?)
    }
}

#[cfg(target_arch = "wasm32")]
impl HttpClient {
    pub async fn get(&self, url: &str) -> Result<String> {
        let response = gloo_net::http::Request::get(url)
            .header("X-Device-UUID", &self.uuid)
            .send()
            .await
            .map_err(|e| anyhow!("Request failed: {}", e))?;
        Ok(response.text()
            .await
            .map_err(|e| anyhow!("Failed to get text: {}", e))?)
    }
}
