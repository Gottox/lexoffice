use crate::request::Request;
use crate::Error;
use crate::Result;
use derive_more::{Display, From, FromStr};
use reqwest::Method;
use reqwest::RequestBuilder;
use reqwest::Url;
use typed_builder::TypedBuilder;

static BASE_URL: &str = "https://api.lexware.io/v1";
//static BASE_URL: &str = "http://127.0.0.1:8100";

/// Represents an API Key
#[derive(Clone, Debug, Display, FromStr, From)]
pub struct ApiKey(String);

impl ApiKey {
    /// Loads the API key from the `LEXOFFICE_KEY` environment variable
    #[cfg(feature = "env")]
    pub fn from_env() -> Result<Self> {
        let env = std::env::var("LEXOFFICE_KEY")?;
        Ok(Self::from(env))
    }

    /// Loads the API key from a specified file.
    #[cfg(feature = "fs")]
    pub async fn from_file(file_name: &std::path::Path) -> Result<Self> {
        let contents = tokio::fs::read_to_string(file_name).await?;
        let contents = contents.trim();
        Ok(contents.parse().unwrap())
    }

    /// Loads the API key from the home folder containing a `~/.lexoffice` file
    #[cfg(all(feature = "fs", feature = "env"))]
    pub async fn from_home() -> Result<Self> {
        use std::env;

        let home = env::var("HOME")?;
        let mut file_name = std::path::PathBuf::from(home);
        file_name.push(".lexoffice");

        Self::from_file(&file_name).await
    }

    /// This function tries to load a key from the following sources in
    /// this order:
    /// 1. The `LEXOFFICE_KEY` environment variable
    /// 2. The `~/.lexoffice` file containing the key
    pub async fn try_default() -> Result<Self> {
        cfg_if::cfg_if! {
            if #[cfg(feature = "env")] {
                if let Ok(key) = Self::from_env() {
                    return Ok(key);
                }
            }
        }

        cfg_if::cfg_if! {
            if #[cfg(all(feature = "fs", feature = "env"))] {
                if let Ok(key) = Self::from_home().await {
                    return Ok(key);
                }
            }
        }

        Err(Error::FailedToLoadApiKey)
    }
}

impl From<&str> for ApiKey {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn default_client() -> reqwest::Client {
    let user_agent = concat!(
        "rust-",
        env!("CARGO_PKG_NAME"),
        "/",
        env!("CARGO_PKG_VERSION"),
    );

    reqwest::Client::builder()
        .user_agent(user_agent)
        .build()
        .unwrap()
}

#[cfg(target_arch = "wasm32")]
fn default_client() -> reqwest::Client {
    reqwest::Client::builder().build().unwrap()
}

/// The Client struct for LexOffice
#[derive(Debug, Clone, TypedBuilder)]
#[builder(doc)]
pub struct Client {
    api_key: ApiKey,

    #[builder(default=default_client())]
    http_client: reqwest::Client,

    #[builder(default=Url::parse(BASE_URL).unwrap())]
    base_url: Url,
}

impl Client {
    /// Creates a new Client with an API key.
    pub fn new<T: Into<ApiKey>>(api_key: T) -> Self {
        Self::builder().api_key(api_key.into()).build()
    }

    /// Creates a new request.
    pub fn request<T: Clone>(&self) -> Request<T> {
        Request::new(self.clone())
    }

    pub(crate) fn http_builder(
        &self,
        method: Method,
        url: Url,
    ) -> RequestBuilder {
        // Not using Request::bearer_auth() here as it's not yet available for
        // wasm
        let bearer = format!("Bearer {}", &self.api_key);

        self.http_client
            .request(method, url)
            .header(reqwest::header::AUTHORIZATION, bearer)
    }

    /// Returns the base Url used by this client
    pub fn base_url(&self) -> &Url {
        &self.base_url
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_key_from_string() {
        let key = ApiKey::from("test-api-key-123");
        assert_eq!(key.to_string(), "test-api-key-123");
    }

    #[test]
    fn test_api_key_from_str() {
        let key: ApiKey = "another-test-key".into();
        assert_eq!(key.to_string(), "another-test-key");
    }

    #[test]
    fn test_client_new() {
        let api_key = "test-key";
        let client = Client::new(api_key);

        // Verify the client is created successfully
        assert_eq!(client.base_url().as_str(), BASE_URL);
    }

    #[test]
    fn test_client_base_url() {
        let client = Client::new("test-key");
        assert_eq!(client.base_url().as_str(), "https://api.lexware.io/v1");
    }

    #[test]
    fn test_client_custom_base_url() {
        let custom_url = Url::parse("https://custom.api.example.com/v2").unwrap();
        let client = Client::builder()
            .api_key(ApiKey::from("test-key"))
            .base_url(custom_url.clone())
            .build();

        assert_eq!(client.base_url(), &custom_url);
    }

    #[cfg(feature = "env")]
    #[test]
    fn test_api_key_from_env_missing() {
        std::env::remove_var("LEXOFFICE_KEY");
        let result = ApiKey::from_env();
        assert!(result.is_err());
    }

    #[cfg(feature = "env")]
    #[test]
    fn test_api_key_from_env_present() {
        std::env::set_var("LEXOFFICE_KEY", "env-test-key");
        let result = ApiKey::from_env();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().to_string(), "env-test-key");
        std::env::remove_var("LEXOFFICE_KEY");
    }
}
