use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Language {
  id: usize,
  name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
  id: usize,
  description: String
}
