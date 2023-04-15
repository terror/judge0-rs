#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("Error making request")]
  Request(#[from] reqwest::Error),
}
