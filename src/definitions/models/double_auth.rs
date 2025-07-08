use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct DoubleAuth {
  #[serde(rename = "cn")]
  name: String,
  #[serde(rename = "cv")]
  value: String,
}
