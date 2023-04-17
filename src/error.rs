#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("Failed to make request")]
  Request(#[from] reqwest::Error),
  #[error("Failed to serialize/deserialize json")]
  Serde(#[from] serde_json::Error),
  #[error("Invalid header name: {0}")]
  HeaderName(String),
  #[error("Invalid header value: {0}")]
  HeaderValue(String),
}
