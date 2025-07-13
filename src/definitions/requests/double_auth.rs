use serde::Serialize;
use serde_with::{base64::Base64, serde_as};

#[serde_as]
#[derive(Debug, Serialize)]
pub struct DoubleAuthSolveRequest {
  #[serde_as(as = "Base64")]
  #[serde(rename = "choix")]
  pub answer: String,
}
