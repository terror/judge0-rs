use {
  crate::error::Error,
  chrono::prelude::*,
  http::HeaderMap,
  http::Method,
  serde::de::DeserializeOwned,
  serde::{Deserialize, Serialize},
  serde_json::Value,
};

mod client;
mod error;
mod model;

type Result<T = (), E = Error> = std::result::Result<T, E>;

pub use {client::Client, model::*};
