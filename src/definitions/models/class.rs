use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Class {
  pub id: u32,
  pub code: String,
  #[serde(rename = "libelle")]
  pub label: String,
  #[serde(rename = "estNote")]
  pub is_graded: u32,
}
