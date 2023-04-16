use super::*;

#[derive(Debug)]
pub struct Client {
  base_url: String,
  client: reqwest::Client,
  config: Config,
}

impl Client {
  /// Create a new client.
  ///
  /// ```rust
  /// use judge0_rs::Client;
  ///
  /// let client = Client::new("http://localhost:2358");
  /// ```
  pub fn new(base_url: &str) -> Client {
    Self {
      base_url: base_url.to_owned(),
      client: reqwest::Client::new(),
      config: Config::default(),
    }
  }

  /// Configure the client.
  ///
  /// ```rust
  /// use judge0_rs::{Client, Config};
  ///
  /// let client = Client::new("http://localhost:2358").configure(Config::default());
  /// ```
  pub fn configure(self, config: Config) -> Self {
    Self { config, ..self }
  }

  /// Check if your authentication token is valid.
  ///
  /// ```rust
  /// use judge0_rs::{Client, Config};
  ///
  /// let client = Client::new("http://localhost:2358").configure(Config::default());
  ///
  /// assert!(client.authenticate().await.is_ok());
  /// ```
  pub async fn authenticate(self) -> Result {
    self.request("/authenticate", Method::POST).await
  }

  /// Check if your authorization token is valid.
  ///
  /// ```rust
  /// use judge0_rs::{Client, Config};
  ///
  /// let client = Client::new("http://localhost:2358").configure(Config::default());
  ///
  /// assert!(client.authorize().await.is_ok());
  /// ```
  pub async fn autorize(self) -> Result {
    self.request("/authorize", Method::POST).await
  }

  /// Get active languages.
  ///
  /// ```rust
  /// use judge0_rs::{Client, Config};
  ///
  /// let client = Client::new("http://localhost:2358").configure(Config::default());
  ///
  /// let languages = client.get_languages().await.unwrap();
  /// ```
  pub async fn get_languages(self) -> Result<Vec<Language>> {
    self
      .request::<Vec<Language>>("/languages", Method::GET)
      .await
  }

  /// Get active and archived languages.
  ///
  /// ```rust
  /// use judge0_rs::{Client, Config};
  ///
  /// let client = Client::new("http://localhost:2358").configure(Config::default());
  ///
  /// let languages = client.get_all_languages().await.unwrap();
  /// ```
  pub async fn get_all_languages(self) -> Result<Vec<Language>> {
    self
      .request::<Vec<Language>>("/languages/all", Method::GET)
      .await
  }

  /// Get a single active language by identifier.
  ///
  /// ```rust
  /// use judge0_rs::{Client, Config};
  ///
  /// let client = Client::new("http://localhost:2358").configure(Config::default());
  ///
  /// let language = client.get_language(1).await.unwrap();
  /// ```
  pub async fn get_language(self, id: usize) -> Result<Language> {
    self
      .request::<Language>(&format!("/languages/{id}"), Method::GET)
      .await
  }

  /// Get all statuses.
  ///
  /// ```rust
  /// use judge0_rs::{Client, Config};
  ///
  /// let client = Client::new("http://localhost:2358").configure(Config::default());
  ///
  /// let statuses = client.get_statuses().await.unwrap();
  /// ```
  pub async fn get_statuses(self) -> Result<Vec<Status>> {
    self.request::<Vec<Status>>("/statuses", Method::GET).await
  }

  /// Get about information.
  ///
  /// ```rust
  /// use judge0_rs::{Client, Config};
  ///
  /// let client = Client::new("http://localhost:2358").configure(Config::default());
  ///
  /// let about = client.get_about().await.unwrap();
  /// ```
  pub async fn get_about(self) -> Result<About> {
    self.request::<About>("/statuses", Method::GET).await
  }

  /// Create a submission.
  ///
  /// ```rust
  /// use judge0_rs::{Client, Config, Submission};
  ///
  /// let client = Client::new("http://localhost:2358").configure(Config::default());
  ///
  /// let submission = Submission {
  ///   source_code: "print(Hello, world)".into(),
  ///   language_id: 1,
  ///   ..Submission::default()
  /// };
  ///
  /// let result = client.create_submission(submission).await.unwrap();
  /// ```
  pub async fn create_submission(
    self,
    submission: Submission,
  ) -> Result<Value> {
    self
      .request_with_body::<Value, Submission>(
        &format!(
          "/submissions?base64_encoded={}&wait={}",
          self.config.base64_encoded, self.config.wait
        ),
        Method::POST,
        submission,
      )
      .await
  }

  /// Get a single submission by token.
  ///
  /// ```rust
  /// use judge0_rs::{Client, Config, Submission};
  ///
  /// let client = Client::new("http://localhost:2358").configure(Config::default());
  ///
  /// let submission = Submission {
  ///   source_code: "print(Hello, world)".into(),
  ///   language_id: 1,
  ///   ..Submission::default()
  /// };
  ///
  /// let result = client.create_submission(submission).await.unwrap();
  ///
  /// let submission = client.get_submission(result["token"], None).await.unwrap();
  /// ```
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
          self.config.base64_encoded,
          self.config.wait
        ),
        Method::GET,
      )
      .await
  }

  /// Delete a single submission by token.
  ///
  /// ```rust
  /// use judge0_rs::{Client, Config, Submission};
  ///
  /// let client = Client::new("http://localhost:2358").configure(Config::default());
  ///
  /// let submission = Submission {
  ///   source_code: "print(Hello, world)".into(),
  ///   language_id: 1,
  ///   ..Submission::default()
  /// };
  ///
  /// let result = client.create_submission(submission).await.unwrap();
  ///
  /// let submission = client.delete_submission(result["token"], None).await.unwrap();
  /// ```
  pub async fn delete_submission(
    self,
    token: &str,
    fields: Option<&str>,
  ) -> Result<Submission> {
    self
      .request::<Submission>(
        &format!("/submissions/{token}?fields={}", fields.unwrap_or("*"),),
        Method::DELETE,
      )
      .await
  }

  /// Create a batch submission.
  ///
  /// ```rust
  /// use judge0_rs::{Client, Config, Submission};
  ///
  /// let client = Client::new("http://localhost:2358").configure(Config::default());
  ///
  /// let submissions = vec![
  ///   Submission {
  ///     source_code: "print("foo")".into(),
  ///     language_id: 1,
  ///     ..Submission::default()
  ///   },
  ///   Submission {
  ///     source_code: "print("bar")".into(),
  ///     language_id: 1,
  ///     ..Submission::default()
  ///   },
  ///   Submission {
  ///     source_code: "print("baz")".into(),
  ///     language_id: 1,
  ///     ..Submission::default()
  ///   },
  /// ];
  ///
  /// let result = client.batch_submit(submissions).await.unwrap();
  /// ```
  pub async fn batch_submit(
    self,
    submissions: Vec<Submission>,
  ) -> Result<Vec<Value>> {
    self
      .request_with_body::<Vec<Value>, Vec<Submission>>(
        &format!(
          "/submissions/batch?base64_encoded={}",
          self.config.base64_encoded
        ),
        Method::POST,
        submissions,
      )
      .await
  }

  /// Get a batch submission.
  ///
  /// ```rust
  /// use judge0_rs::{Client, Config, Submission};
  ///
  /// let client = Client::new("http://localhost:2358").configure(Config::default());
  ///
  /// let submissions = vec![
  ///   Submission {
  ///     source_code: "print("foo")".into(),
  ///     language_id: 1,
  ///     ..Submission::default()
  ///   },
  ///   Submission {
  ///     source_code: "print("bar")".into(),
  ///     language_id: 1,
  ///     ..Submission::default()
  ///   },
  ///   Submission {
  ///     source_code: "print("baz")".into(),
  ///     language_id: 1,
  ///     ..Submission::default()
  ///   },
  /// ];
  ///
  /// let result = client.batch_submit(submissions).await.unwrap();
  ///
  /// let tokens = result.iter().map(|value| value["token"]).collect::<Vec<String>>();
  ///
  /// let batch_submission = get_back_submission(tokens.join(","), None).await.unwrap();
  /// ```
  pub async fn get_batch_submission(
    self,
    tokens: Vec<&str>,
    fields: Option<&str>,
  ) -> Result<Vec<Submission>> {
    self
      .request::<Vec<Submission>>(
        &format!(
          "/submission/batch?tokens={}&base64_encoded={}&fields={}",
          tokens.join(","),
          self.config.base64_encoded,
          fields.unwrap_or("*")
        ),
        Method::GET,
      )
      .await
  }

  /// Build pre-defined headers for each request.
  fn headers(&self) -> Result<HeaderMap> {
    let mut headers = HeaderMap::new();

    headers.insert(
      "content-type",
      HeaderValue::from_str("application/json").unwrap(),
    );

    if let Some(authentication_token) = &self.config.authentication_token {
      headers.insert(
        HeaderName::from_str(&self.config.authentication_header_name).unwrap(),
        HeaderValue::from_str(authentication_token).unwrap(),
      );
    }

    if let Some(authorization_token) = &self.config.authorization_token {
      headers.insert(
        HeaderName::from_str(&self.config.authorization_header_name).unwrap(),
        HeaderValue::from_str(authorization_token).unwrap(),
      );
    }

    Ok(headers)
  }

  /// Make an asynchronous request.
  async fn request<T: DeserializeOwned>(
    &self,
    endpoint: &str,
    method: Method,
  ) -> Result<T> {
    Ok(
      self
        .client
        .request(method, format!("{}{}", self.base_url, endpoint))
        .headers(self.headers().unwrap())
        .send()
        .await?
        .json::<T>()
        .await?,
    )
  }

  /// Make an asynchronous request with a body.
  async fn request_with_body<T: DeserializeOwned, B: Serialize>(
    &self,
    endpoint: &str,
    method: Method,
    body: B,
  ) -> Result<T> {
    Ok(
      self
        .client
        .request(method, format!("{}{}", self.base_url, endpoint))
        .headers(self.headers().unwrap())
        .body(serde_json::to_string(&body)?)
        .send()
        .await?
        .json::<T>()
        .await?,
    )
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
