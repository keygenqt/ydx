use anyhow::{Result, anyhow};
use once_cell::sync::OnceCell;

const ERROR_JSON: &str = r#"{"code": 500, "message": "Internal error" }"#;

static HTTP_CLIENT: OnceCell<HttpClient> = OnceCell::new();

pub struct HttpClient {
    base_url: String,
    uuid: String,
}

impl HttpClient {
    pub fn init(base_url: String, uuid: String) {
        HTTP_CLIENT.set(Self { base_url, uuid }).ok();
    }

    pub fn instance() -> &'static Self {
        HTTP_CLIENT.get().expect("HttpClient not initialized")
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl HttpClient {
    pub async fn get(&self, path: &str) -> Result<String> {
        let response = reqwest::Client::new()
            .get(&format!("{}{}", self.base_url, path))
            .header("X-Device-UUID", &self.uuid)
            .send()
            .await
            .map_err(|e| {
                eprintln!("Request failed for URL {}: {}", path, e);
                anyhow!(ERROR_JSON)
            })?;

        let text = response.text().await.map_err(|e| {
            eprintln!("Failed to read response body from {}: {}", path, e);
            anyhow!(ERROR_JSON)
        })?;

        Ok(text)
    }

    pub async fn post(&self, path: &str, body: serde_json::Value) -> Result<String> {
        let response = reqwest::Client::new()
            .post(&format!("{}{}", self.base_url, path))
            .header("X-Device-UUID", &self.uuid)
            .header("Content-Type", "application/json")
            .body(body.to_string())
            .send()
            .await
            .map_err(|e| {
                eprintln!("POST request failed for {}: {}", path, e);
                anyhow!(ERROR_JSON)
            })?;

        response.text().await.map_err(|e| {
            eprintln!("Failed to read response body from {}: {}", path, e);
            anyhow!(ERROR_JSON)
        })
    }

    pub async fn delete(&self, path: &str) -> Result<String> {
        let response = reqwest::Client::new()
            .delete(&format!("{}{}", self.base_url, path))
            .header("X-Device-UUID", &self.uuid)
            .send()
            .await
            .map_err(|e| {
                eprintln!("DELETE request failed for {}: {}", path, e);
                anyhow!(ERROR_JSON)
            })?;

        response.text().await.map_err(|e| {
            eprintln!("Failed to read response body from {}: {}", path, e);
            anyhow!(ERROR_JSON)
        })
    }
}

#[cfg(target_arch = "wasm32")]
impl HttpClient {
    pub async fn get(&self, path: &str) -> Result<String> {
        let response = gloo_net::http::Request::get(&format!("{}{}", self.base_url, path))
            .header("X-Device-UUID", &self.uuid)
            .send()
            .await
            .map_err(|e| {
                web_sys::console::error_1(
                    &format!("Request failed for URL {}: {:?}", path, e).into(),
                );
                anyhow!(ERROR_JSON)
            })?;

        let text = response.text().await.map_err(|e| {
            web_sys::console::error_1(
                &format!("Failed to read response body from {}: {:?}", path, e).into(),
            );
            anyhow!(ERROR_JSON)
        })?;

        Ok(text)
    }

    pub async fn post(&self, path: &str, body: serde_json::Value) -> Result<String> {
        let response = gloo_net::http::Request::post(&format!("{}{}", self.base_url, path))
            .header("X-Device-UUID", &self.uuid)
            .header("Content-Type", "application/json")
            .body(body.to_string())?
            .send()
            .await
            .map_err(|e| {
                web_sys::console::error_1(
                    &format!("POST request failed for {}: {:?}", path, e).into(),
                );
                anyhow!(ERROR_JSON)
            })?;

        response.text().await.map_err(|e| {
            web_sys::console::error_1(
                &format!("Failed to read response body from {}: {:?}", path, e).into(),
            );
            anyhow!(ERROR_JSON)
        })
    }

    pub async fn delete(&self, path: &str) -> Result<String> {
        let response = gloo_net::http::Request::delete(&format!("{}{}", self.base_url, path))
            .header("X-Device-UUID", &self.uuid)
            .send()
            .await
            .map_err(|e| {
                web_sys::console::error_1(
                    &format!("DELETE request failed for {}: {:?}", path, e).into(),
                );
                anyhow!(ERROR_JSON)
            })?;

        response.text().await.map_err(|e| {
            web_sys::console::error_1(
                &format!("Failed to read response body from {}: {:?}", path, e).into(),
            );
            anyhow!(ERROR_JSON)
        })
    }
}
