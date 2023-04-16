#[derive(Debug)]
pub struct Config {
  /// X-Auth-Token is the default header field name, but administrators of the
  /// judge0 instance you are using can change this default field name.
  pub authentication_header_name: String,

  /// Administrators of judge0 can configure judge0 to require you to have an
  /// authentication token (a.k.a. API key). If that is the case with the
  /// instance of judge0 you are using, then you should provide X-Auth-Token
  /// header field in every API request.
  pub authentication_token: Option<String>,

  /// X-Auth-User is default header field name, but administrators of judge0
  /// instance you are using can change this default field name.
  pub authorization_header_name: String,

  /// To issue some API calls you need to be authorized. For example, you need
  /// to be authorized to list all submissions on Judge0.
  pub authorization_token: Option<String>,

  /// Set to true if you want to send base64 encoded data to judge0.
  pub base64_encoded: bool,

  /// Instead of checking submission status by making another request, you can
  /// set the wait query parameter to true which will enable you to get
  /// submission status immediately as part of response to the request you
  /// made.
  ///
  /// n.b The use of wait=true feature is not recommended because it does not
  /// scale well.
  pub wait: bool,
}

impl Default for Config {
  fn default() -> Self {
    Self {
      authentication_header_name: String::from("X-Auth-Token"),
      authentication_token: None,
      authorization_header_name: String::from("X-Auth-User"),
      authorization_token: None,
      base64_encoded: false,
      wait: false,
    }
  }
}
