#[derive(Debug, Clone)]
pub struct Authentication {
  pub token: Option<String>,
  pub username: String,
  pub password: String,
  pub device_uuid: String,
  pub access_token: Option<String>,
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
      token: None,
      username,
      password,
      device_uuid,
      access_token: None,
    }
  }
}
