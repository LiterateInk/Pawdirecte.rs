use crate::definitions::models::DoubleAuth;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct LoginRequest {
  #[serde(rename = "identifiant")]
  pub username: String,

  /// - Should be URL encoded.
  /// - Should be exactly `"???"` when providing an access token.
  #[serde(rename = "motdepasse")]
  pub password: String,

  #[serde(rename = "uuid")]
  pub device_uuid: String,

  /// Always `false`, only `true` when you authenticate using an access token.
  #[serde(rename = "isReLogin")]
  pub is_reauth: bool,

  #[serde(rename = "sesouvenirdemoi")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub remember_me: Option<bool>,

  #[serde(rename = "fa")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub double_auth: Option<Vec<DoubleAuth>>,

  #[serde(rename = "typeCompte")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub account_type: Option<u16>,

  #[serde(rename = "accesstoken")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub access_token: Option<String>,
}
