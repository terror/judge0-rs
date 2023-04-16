use super::*;

#[derive(Debug)]
pub struct Client {
  base_url: String,
  client: reqwest::Client,
  base64_encoded: bool,
  wait: bool,
}

impl Client {
  /// Create a new client.
  pub fn new(base_url: &str) -> Client {
    Self {
      base_url: base_url.to_owned(),
      client: reqwest::Client::new(),
      base64_encoded: false,
      wait: false,
    }
  }

  /// Set to true if you want to send Base64 encoded data to Judge0.
  pub fn set_base64_encoded(self, base64_encoded: bool) -> Self {
    Self {
      base64_encoded,
      ..self
    }
  }

  /// Instead of checking submission status by making another request, you can
  /// set the wait query parameter to true which will enable you to get
  /// submission status immediately as part of response to the request you made.
  ///
  /// n.b The use of wait=true feature is not recommended because it does not
  /// scale well.
  pub fn set_wait(self, wait: bool) -> Self {
    Self { wait, ..self }
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

  /// Get a single submission by token.
  pub async fn get_submission(
    self,
    token: &str,
    fields: Option<&str>,
  ) -> Result<Submission> {
    self
      .request::<Submission>(
        &format!(
          "/submissions/{token}?base64_encoded={}&wait={}&fields={}",
          fields.unwrap_or("*"),
          self.base64_encoded,
          self.wait
        ),
        Method::GET,
        None,
      )
      .await
  }

  /// Delete a single submission by token.
  pub async fn delete_submission(
    self,
    token: &str,
    fields: Option<&str>,
    headers: Option<HeaderMap>,
  ) -> Result<Submission> {
    self
      .request::<Submission>(
        &format!("/submissions/{token}?fields={}", fields.unwrap_or("*"),),
        Method::DELETE,
        headers,
      )
      .await
  }

  /// Create a batch submission.
  pub async fn batch_submit(
    self,
    submissions: Vec<Submission>,
    headers: Option<HeaderMap>,
  ) -> Result<Vec<Value>> {
    self
      .request_with_body::<Vec<Value>, Vec<Submission>>(
        &format!("/submissions/batch?base64_encoded={}", self.base64_encoded),
        Method::POST,
        headers,
        submissions,
      )
      .await
  }

  /// Get a batch submission.
  pub async fn get_batch_submission(
    self,
    tokens: Vec<&str>,
    fields: &str,
    headers: Option<HeaderMap>,
  ) -> Result<Vec<Submission>> {
    self
      .request::<Vec<Submission>>(
        &format!(
          "/submission/batch?tokens={}&base64_encoded={}&fields={}",
          tokens.join(","),
          self.base64_encoded,
          fields
        ),
        Method::GET,
        headers,
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

  /// Make an asynchronous request with a body.
  async fn request_with_body<T: DeserializeOwned, B: Serialize>(
    &self,
    endpoint: &str,
    method: Method,
    headers: Option<HeaderMap>,
    body: B,
  ) -> Result<T> {
    let mut request = self
      .client
      .request(method, format!("{}{}", self.base_url, endpoint));

    if let Some(headers) = headers {
      request = request.headers(headers);
    }

    request = request.body(serde_json::to_string(&body)?);

    Ok(request.send().await?.json::<T>().await?)
  }
}

#[cfg(test)]
mod tests {
  use {
    super::*,
    mockito::{Server, ServerGuard},
  };

  struct TestContext {
    server: ServerGuard,
  }

  impl TestContext {
    async fn new() -> Self {
      Self {
        server: Server::new_async().await,
      }
    }
  }

  #[tokio::test(flavor = "multi_thread")]
  async fn languages_ok() {
    let TestContext { mut server } = TestContext::new().await;

    let client = Client::new(&server.url());

    let body = r#" [
      { "id": 45, "name": "Assembly (NASM 2.14.02)" },
      { "id": 46, "name": "Bash (5.0.0)" },
      { "id": 47, "name": "Basic (FBC 1.07.1)" }
    ]"#;

    let mock = server
      .mock("GET", "/languages")
      .with_status(200)
      .with_header("content-type", "application/json")
      .with_body(body)
      .create();

    let languages = client.get_languages().await.unwrap();

    assert_eq!(
      languages,
      serde_json::from_str::<Vec<Language>>(body).unwrap()
    );

    mock.assert();
  }

  #[tokio::test(flavor = "multi_thread")]
  async fn language_ok() {
    let TestContext { mut server } = TestContext::new().await;

    let client = Client::new(&server.url());

    let body = r#"{
      "id": 1,
      "name": "Bash (4.4)",
      "is_archived": true,
      "source_file": "script.sh",
      "compile_cmd": null,
      "run_cmd": "/usr/local/bash-4.4/bin/bash script.sh"
    }"#;

    let mock = server
      .mock("GET", "/languages/1")
      .with_status(200)
      .with_header("content-type", "application/json")
      .with_body(body)
      .create();

    let language = client.get_language(1).await.unwrap();

    assert_eq!(language, serde_json::from_str::<Language>(body).unwrap());

    mock.assert();
  }
}
