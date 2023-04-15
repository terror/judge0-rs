use http::HeaderMap;
use serde::de::DeserializeOwned;

use super::*;

#[derive(Debug)]
pub struct Client<'a> {
  base_url: &'a str,
  client: reqwest::Client,
}

impl<'a> Client<'a> {
  pub fn new(base_url: &'a str) -> Client {
    Self {
      base_url,
      client: reqwest::Client::new(),
    }
  }

  /// Get active languages.
  pub async fn languages(self) -> Result<Vec<Language>> {
    self
      .request::<Vec<Language>>("/languages", Method::GET, None)
      .await
  }

  /// Get a single active language by identifier.
  pub async fn language(self, id: usize) -> Result<Language> {
    self
      .request::<Language>(&format!("/languages/{id}"), Method::GET, None)
      .await
  }

  /// Get all statuses.
  pub async fn statuses(self) -> Result<Vec<Status>> {
    self
      .request::<Vec<Status>>("/statuses", Method::GET, None)
      .await
  }

  async fn request<T: DeserializeOwned>(
    &self,
    endpoint: &str,
    method: Method,
    headers: Option<HeaderMap>,
  ) -> Result<T> {
    let mut request = self
      .client
      .request(method, format!("{}{}", self.base_url, endpoint));

    if let Some(headers) = headers {
      request = request.headers(headers);
    }

    Ok(request.send().await?.json::<T>().await?)
  }
}
