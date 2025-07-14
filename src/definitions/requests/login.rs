use crate::definitions::responses::DoubleAuthSolveResponse;
use serde::Serialize;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Serialize)]
pub struct LoginRequest {
  #[serde(rename = "identifiant")]
  pub username: String,

  /// - Should be exactly `"???"` when providing an access token.
  #[serde(rename = "motdepasse")]
  pub password: String,

  #[serde(rename = "uuid")]
  pub device_uuid: String,

  /// Always `false`, only `true` when you authenticate using an access token.
  #[serde(rename = "isReLogin")]
  pub is_reauth: bool,

  #[serde(rename = "sesouvenirdemoi")]
  pub remember_me: Option<bool>,

  #[serde(rename = "fa")]
  pub double_auth: Option<Vec<DoubleAuthSolveResponse>>,

  #[serde(rename = "typeCompte")]
  pub account_type: Option<String>,

  #[serde(rename = "accesstoken")]
  pub access_token: Option<String>,
}
