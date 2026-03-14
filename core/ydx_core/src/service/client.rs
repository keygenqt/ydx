use anyhow::{Result, anyhow};
use once_cell::sync::OnceCell;
use serde_json::Value;

const ERROR_JSON: &str = r#"{"code": 500, "message": "Internal error" }"#;

static HTTP_CLIENT: OnceCell<HttpClient> = OnceCell::new();

// Enum для разных платформ
pub enum PlatformClient {
    #[cfg(any(feature = "ffi", feature = "sffi"))]
    Ffi(ClientFFI),
    #[cfg(feature = "wasm")]
    Wasm(ClientWASM),
}

pub struct HttpClient {
    platform: PlatformClient,
}

#[allow(unused_variables)]
impl HttpClient {
    pub fn init(base_url: &str, uuid: &str) {
        #[cfg(any(feature = "ffi", feature = "sffi"))]
        {
            let platform = PlatformClient::Ffi(ClientFFI::new(base_url, uuid));
            HTTP_CLIENT.set(Self { platform }).ok();
        }

        #[cfg(feature = "wasm")]
        {
            let platform = PlatformClient::Wasm(ClientWASM::new(base_url, uuid));
            HTTP_CLIENT.set(Self { platform }).ok();
        }

        #[cfg(not(any(feature = "sffi", feature = "ffi", feature = "wasm")))]
        compile_error!("Either 'sffi/ffi' or 'wasm' feature must be enabled");
    }

    pub fn instance() -> &'static Self {
        HTTP_CLIENT.get().expect("HttpClient not initialized")
    }

    pub async fn get(&self, path: &str, token: Option<String>) -> Result<String> {
        match &self.platform {
            #[cfg(any(feature = "ffi", feature = "sffi"))]
            PlatformClient::Ffi(client) => client.get(path, token).await,
            #[cfg(feature = "wasm")]
            PlatformClient::Wasm(client) => client.get(path).await,
        }
    }

    pub async fn post(&self, path: &str, token: Option<String>, body: Value) -> Result<String> {
        match &self.platform {
            #[cfg(any(feature = "ffi", feature = "sffi"))]
            PlatformClient::Ffi(client) => client.post(path, body, token).await,
            #[cfg(feature = "wasm")]
            PlatformClient::Wasm(client) => client.post(path, body).await,
        }
    }

    pub async fn delete(&self, path: &str, token: Option<String>) -> Result<String> {
        match &self.platform {
            #[cfg(any(feature = "ffi", feature = "sffi"))]
            PlatformClient::Ffi(client) => client.delete(path, token).await,
            #[cfg(feature = "wasm")]
            PlatformClient::Wasm(client) => client.delete(path).await,
        }
    }
}

// FFI реализация
#[cfg(any(feature = "ffi", feature = "sffi"))]
trait RequestBuilderExt: Sized {
    fn with_optional_auth(self, token: Option<String>) -> Self;
}

#[cfg(any(feature = "ffi", feature = "sffi"))]
impl RequestBuilderExt for reqwest::RequestBuilder {
    fn with_optional_auth(self, token: Option<String>) -> Self {
        if let Some(token) = token {
            self.header("Authorization", format!("Bearer {}", token))
        } else {
            self
        }
    }
}

#[cfg(any(feature = "ffi", feature = "sffi"))]
pub struct ClientFFI {
    base_url: String,
    uuid: String,
}

#[cfg(any(feature = "ffi", feature = "sffi"))]
impl ClientFFI {
    pub fn new(base_url: &str, uuid: &str) -> Self {
        Self {
            base_url: base_url.into(),
            uuid: uuid.into(),
        }
    }

    pub async fn get(&self, path: &str, token: Option<String>) -> Result<String> {
        let response = reqwest::Client::new()
            .get(&format!("{}{}", self.base_url, path))
            .header("X-Device-UUID", &self.uuid)
            .with_optional_auth(token)
            .send()
            .await
            .map_err(|e| {
                eprintln!("Request failed for URL {}/{}: {}", self.base_url, path, e);
                anyhow!(ERROR_JSON)
            })?;

        response.text().await.map_err(|e| {
            eprintln!("Failed to read response body from {}: {}", path, e);
            anyhow!(ERROR_JSON)
        })
    }

    pub async fn post(&self, path: &str, body: Value, token: Option<String>) -> Result<String> {
        let response = reqwest::Client::new()
            .post(&format!("{}{}", self.base_url, path))
            .header("X-Device-UUID", &self.uuid)
            .header("Content-Type", "application/json")
            .with_optional_auth(token)
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

    pub async fn delete(&self, path: &str, token: Option<String>) -> Result<String> {
        let response = reqwest::Client::new()
            .delete(&format!("{}{}", self.base_url, path))
            .header("X-Device-UUID", &self.uuid)
            .with_optional_auth(token)
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

// WASM реализация
#[cfg(feature = "wasm")]
pub struct ClientWASM {
    base_url: String,
    uuid: String,
}

#[cfg(feature = "wasm")]
impl ClientWASM {
    pub fn new(base_url: &str, uuid: &str) -> Self {
        Self {
            base_url: base_url.into(),
            uuid: uuid.into(),
        }
    }

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

        response.text().await.map_err(|e| {
            web_sys::console::error_1(
                &format!("Failed to read response body from {}: {:?}", path, e).into(),
            );
            anyhow!(ERROR_JSON)
        })
    }

    pub async fn post(&self, path: &str, body: Value) -> Result<String> {
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
