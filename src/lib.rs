// pub struct AuthenticationDoubleAuth {
//     cn: String,
//     cv: String,
// }

// pub struct Authentication {
//     username: String,
//     password: String,
//     device_uuid: String,
//     double_auth: Option<AuthenticationDoubleAuth>,
// }

use crate::definitions::models::Account;

// impl Authentication {}
pub mod definitions;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("unauthenticated session")]
    WrongLoginState(),
}

#[derive(Debug)]
pub struct LoginManager {
    pub username: String,
    pub password: String,
    pub device_uuid: String,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub requires_2fa: bool,
    login_response: Option<definitions::responses::LoginResponse>,
}

impl LoginManager {
    pub fn from_credentials(
        username: String,
        password: String,
        device_uuid: Option<String>,
    ) -> Self {
        let device_uuid = device_uuid.unwrap_or_else(|| uuid::Uuid::new_v4().into());

        Self {
            username,
            password,
            device_uuid,
            access_token: None,
            refresh_token: None,
            requires_2fa: false,
            login_response: None,
        }
    }

    /// Initializes a session using the given credentials.
    /// `initialize` only performs the first step which is
    /// authenticating but does NOT return accounts nor 2FA challenge.
    ///
    /// See `finalize` for retrieving accounts.
    pub async fn initialize(&self) {}

    pub fn finalize(&self) -> Result<Vec<Account>, Error> {
        if let Some(login_response) = &self.login_response {
            Ok(login_response.accounts.clone())
        } else {
            Err(Error::WrongLoginState())
        }
    }
}
