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
  /// let client = Client::new("http://localhost:2358").configure(Config {
  ///   authentication_token: Some("token".into()),
  ///   ..Default::default()
  /// });
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
  /// let client = Client::new("http://localhost:2358").configure(Config {
  ///   authorization_token: Some("token".into()),
  ///   ..Default::default()
  /// });
  ///
  /// assert!(client.authorize().await.is_ok());
  /// ```
  pub async fn authorize(self) -> Result {
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
    self.request::<About>("/about", Method::GET).await
  }

  /// Get worker information.
  ///
  /// ```rust
  /// use judge0_rs::{Client, Config};
  ///
  /// let client = Client::new("http://localhost:2358").configure(Config::default());
  ///
  /// let workers = client.get_workers().await.unwrap();
  /// ```
  pub async fn get_workers(self) -> Result<Vec<Worker>> {
    self.request::<Vec<Worker>>("/workers", Method::GET).await
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
  /// let tokens = result
  ///   .iter()
  ///   .map(|value| value["token"])
  ///   .collect::<Vec<String>>();
  ///
  /// let batch_submission = get_back_submission(tokens, None).await.unwrap();
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

    let header_name = |name: &str| -> Result<HeaderName> {
      HeaderName::from_str(name)
        .map_err(|_| Error::HeaderName(name.to_string()))
    };

    let header_value = |value: &str| -> Result<HeaderValue> {
      HeaderValue::from_str(value)
        .map_err(|_| Error::HeaderName(value.to_string()))
    };

    headers.insert("content-type", header_value("application/json")?);

    if let Some(authentication_token) = &self.config.authentication_token {
      headers.insert(
        header_name(&self.config.authentication_header_name)?,
        header_value(authentication_token)?,
      );
    }

    if let Some(authorization_token) = &self.config.authorization_token {
      headers.insert(
        header_name(&self.config.authorization_header_name)?,
        header_value(authorization_token)?,
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
        .headers(self.headers()?)
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
        .headers(self.headers()?)
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

  #[tokio::test(flavor = "multi_thread")]
  async fn about_ok() {
    let TestContext { mut server } = TestContext::new().await;

    let client = Client::new(&server.url());

    let body = r#"{
      "version": "1.5.0",
      "homepage": "https://judge0.com",
      "source_code": "https://github.com/judge0/judge0",
      "maintainer": "Herman Zvonimir Došilović <hermanz.dosilovic@gmail.com>"
    }"#;

    let mock = server
      .mock("GET", "/about")
      .with_status(200)
      .with_header("content-type", "application/json")
      .with_body(body)
      .create();

    let about = client.get_about().await.unwrap();

    assert_eq!(about, serde_json::from_str::<About>(body).unwrap());

    mock.assert();
  }

  #[tokio::test(flavor = "multi_thread")]
  async fn workers_ok() {
    let TestContext { mut server } = TestContext::new().await;

    let client = Client::new(&server.url());

    let body = r#"[{
      "queue": "default",
      "size": 0,
      "available": 1,
      "idle": 1,
      "working": 0,
      "paused": 0,
      "failed": 0
    }]"#;

    let mock = server
      .mock("GET", "/workers")
      .with_status(200)
      .with_header("content-type", "application/json")
      .with_body(body)
      .create();

    let workers = client.get_workers().await.unwrap();

    assert_eq!(workers, serde_json::from_str::<Vec<Worker>>(body).unwrap());

    mock.assert();
  }

  #[tokio::test(flavor = "multi_thread")]
  async fn create_submission_ok() {
    let TestContext { mut server } = TestContext::new().await;

    let client = Client::new(&server.url());

    let body = r#"{
      "token": "d85cd024-1548-4165-96c7-7bc88673f194"
    }"#;

    let mock = server
      .mock("POST", "/submissions?base64_encoded=false&wait=false")
      .with_status(201)
      .with_header("content-type", "application/json")
      .with_body(body)
      .create();

    let result = client
      .create_submission(Submission {
        source_code: r#"print("Hello, world!")"#.into(),
        language_id: 1,
        ..Default::default()
      })
      .await
      .unwrap();

    assert_eq!(result, serde_json::from_str::<Value>(body).unwrap());

    mock.assert();
  }

  #[tokio::test(flavor = "multi_thread")]
  async fn create_submission_invalid_language() {
    let TestContext { mut server } = TestContext::new().await;

    let client = Client::new(&server.url());

    let body = r#"{
      "language_id": ["language with id 9000 doesn't exist"]
    }"#;

    let mock = server
      .mock("POST", "/submissions?base64_encoded=false&wait=false")
      .with_status(422)
      .with_header("content-type", "application/json")
      .with_body(body)
      .create();

    let result = client
      .create_submission(Submission {
        source_code: r#"print("Hello, world!")"#.into(),
        language_id: 9000,
        ..Default::default()
      })
      .await
      .unwrap();

    assert_eq!(result, serde_json::from_str::<Value>(body).unwrap());

    mock.assert();
  }

  #[tokio::test(flavor = "multi_thread")]
  async fn create_submission_invalid_wall_time_limit() {
    let TestContext { mut server } = TestContext::new().await;

    let client = Client::new(&server.url());

    let body = r#"{
      "wall_time_limit": ["must be less than or equal to 150"]
    }"#;

    let mock = server
      .mock("POST", "/submissions?base64_encoded=false&wait=false")
      .with_status(422)
      .with_header("content-type", "application/json")
      .with_body(body)
      .create();

    let result = client
      .create_submission(Submission {
        source_code: r#"print("Hello, world!")"#.into(),
        language_id: 4,
        number_of_runs: Some(1),
        stdin: Some("Judge0".into()),
        expected_output: Some("hello, Judge0".into()),
        cpu_time_limit: Some(1.0),
        cpu_extra_time: Some(0.5),
        wall_time_limit: Some(100000.0),
        memory_limit: Some(128000.0),
        stack_limit: Some(128000),
        enable_per_process_and_thread_time_limit: Some(false),
        enable_per_process_and_thread_memory_limit: Some(false),
        max_file_size: Some(1024),
        ..Default::default()
      })
      .await
      .unwrap();

    assert_eq!(result, serde_json::from_str::<Value>(body).unwrap());

    mock.assert();
  }

  #[tokio::test(flavor = "multi_thread")]
  async fn create_submission_invalid_utf8() {
    let TestContext { mut server } = TestContext::new().await;

    let client = Client::new(&server.url()).configure(Config {
      wait: true,
      ..Default::default()
    });

    let body = r#"{
      "token": "fcd0de6d-ee52-4a9d-8a00-6e0d98d394cf",
      "error": "some attributes for this submission cannot be converted to UTF-8, use base64_encoded=true query parameter"
    }"#;

    let mock = server
      .mock("POST", "/submissions?base64_encoded=false&wait=true")
      .with_status(201)
      .with_header("content-type", "application/json")
      .with_body(body)
      .create();

    let result = client
      .create_submission(Submission {
        source_code: r#""print(\"\xFE\")""#.into(),
        language_id: 70,
        ..Default::default()
      })
      .await
      .unwrap();

    assert_eq!(result, serde_json::from_str::<Value>(body).unwrap());

    mock.assert();
  }

  #[tokio::test(flavor = "multi_thread")]
  async fn create_submission_wait_for_finish() {
    let TestContext { mut server } = TestContext::new().await;

    let client = Client::new(&server.url()).configure(Config {
      wait: true,
      ..Default::default()
    });

    let body = r#"{
      "stdout": "hello, Judge0\n",
      "time": "0.001",
      "memory": 380,
      "stderr": null,
      "token": "eb0dd001-66db-47f4-8a69-b736c9bc23f6",
      "compile_output": null,
      "message": null,
      "status": {
        "id": 3,
        "description": "Accepted"
      }
    }"#;

    let mock = server
      .mock("POST", "/submissions?base64_encoded=false&wait=true")
      .with_status(201)
      .with_header("content-type", "application/json")
      .with_body(body)
      .create();

    let result = client
      .create_submission(Submission {
        source_code: r#"
          #include <stdio.h>

          int main(void) {
            char name[10];
            scanf("%s", name);
            printf("hello, %s\n", name);
            return 0;
          }"#
          .into(),
        language_id: 4,
        stdin: Some("Judge0".into()),
        expected_output: Some("hello, Judge0".into()),
        ..Default::default()
      })
      .await
      .unwrap();

    assert_eq!(result, serde_json::from_str::<Value>(body).unwrap());

    mock.assert();
  }
}
