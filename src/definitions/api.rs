#[derive(Debug, serde::Deserialize)]
pub struct APIResponseWrap<T> {
  pub code: u32,
  pub token: Option<String>,
  pub message: Option<String>,
  pub data: T,
}
