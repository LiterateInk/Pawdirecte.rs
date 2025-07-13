use serde::{Deserialize, Serialize};
use serde_with::{base64::Base64, serde_as};

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct DoubleAuthChallengeResponse {
  #[serde_as(as = "Base64")]
  pub question: String,
  #[serde_as(as = "Vec<Base64>")]
  #[serde(rename = "propositions")]
  pub answers: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DoubleAuthSolveResponse {
  #[serde(rename = "cn")]
  name: String,
  #[serde(rename = "cv")]
  value: String,
}
