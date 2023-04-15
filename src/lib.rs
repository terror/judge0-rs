use {
  crate::error::Error,
  http::Method,
  serde::{Deserialize, Serialize},
};

mod client;
mod error;
mod model;

type Result<T = (), E = Error> = std::result::Result<T, E>;

pub use {client::Client, model::*};
