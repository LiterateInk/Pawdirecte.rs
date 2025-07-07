#[derive(Debug, Clone)]
pub struct Authentication {
  pub username: String,
  pub password: String,
  pub device_uuid: String,
  pub access_token: Option<String>,
  pub refresh_token: Option<String>,
}

impl Authentication {
  pub fn from_credentials(
    username: String,
    password: String,
    device_uuid: Option<String>,
  ) -> Self {
    let device_uuid =
      device_uuid.unwrap_or_else(|| uuid::Uuid::new_v4().into());

    Self {
      username,
      password,
      device_uuid,
      access_token: None,
      refresh_token: None,
    }
  }
}
