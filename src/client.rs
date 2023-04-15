use super::*;

#[derive(Debug)]
pub struct Client<'a> {
  base64_encoded: bool,
  base_url: &'a str,
  client: reqwest::Client,
  wait: bool,
}

impl<'a> Client<'a> {
  /// Create a new client.
  pub fn new(base_url: &'a str) -> Client {
    Self {
      base64_encoded: false,
      base_url,
      client: reqwest::Client::new(),
      wait: false,
    }
  }

  /// Set to true if you want to send Base64 encoded data to Judge0.
  pub fn set_base64_encoded(&mut self, base64_encoded: bool) {
    self.base64_encoded = base64_encoded;
  }

  /// Instead of checking submission status by making another request, you can
  /// set the wait query parameter to true which will enable you to get
  /// submission status immediately as part of response to the request you made.
  ///
  /// n.b The use of wait=true feature is not recommended because it does not
  /// scale well.
  pub fn set_wait(&mut self, wait: bool) {
    self.wait = wait;
  }

  /// Get active languages.
  pub async fn get_languages(self) -> Result<Vec<Language>> {
    self
      .request::<Vec<Language>>("/languages", Method::GET, None)
      .await
  }

  /// Get active and archived languages.
  pub async fn get_all_languages(self) -> Result<Vec<Language>> {
    self
      .request::<Vec<Language>>("/languages/all", Method::GET, None)
      .await
  }

  /// Get a single active language by identifier.
  pub async fn get_language(self, id: usize) -> Result<Language> {
    self
      .request::<Language>(&format!("/languages/{id}"), Method::GET, None)
      .await
  }

  /// Get all statuses.
  pub async fn get_statuses(self) -> Result<Vec<Status>> {
    self
      .request::<Vec<Status>>("/statuses", Method::GET, None)
      .await
  }

  /// Get about information.
  pub async fn get_about(self) -> Result<About> {
    self.request::<About>("/statuses", Method::GET, None).await
  }

  /// Create a submission.
  pub async fn create_submission(
    self,
    headers: Option<HeaderMap>,
  ) -> Result<Value> {
    self
      .request::<Value>(
        &format!(
          "/submissions?base64_encoded={}&wait={}",
          self.base64_encoded, self.wait
        ),
        Method::POST,
        headers,
      )
      .await
  }

  /// Get a single submission.
  pub async fn get_submission(
    self,
    token: &str,
    fields: &str,
  ) -> Result<Submission> {
    self
      .request::<Submission>(
        &format!(
          "/submissions/{token}?base64_encoded={}&wait={}&fields={fields}",
          self.base64_encoded, self.wait
        ),
        Method::GET,
        None,
      )
      .await
  }

  /// Make an asynchronous request.
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
