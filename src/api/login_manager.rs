use crate::{
  api::{Authentication, Error, RequestManager},
  definitions::{models::Account, responses::LoginResponse},
};

/// Will handle most of the authentication.
///
/// 1. create an instance using `from_credentials(username, password)`
/// 2. do `initialize()` to attempt to authenticate
/// 3. check `requires_2fa` value
/// 4. if `true`, you have to call `get_2fa_challenge()` to retrieve the
///    challenge you have to solve to authenticate. once done, you have
///    to call `solve_2fa_challenge(answer)`
/// 5. do `finalize()` to retrieve logged in accounts that you can use
///    for further requests
#[derive(Debug)]
pub struct LoginManager {
  pub requires_2fa: bool,
  login_response: Option<LoginResponse>,
  request_manager: RequestManager,
  pub authentication: Authentication,
}

impl LoginManager {
  pub fn from_credentials(
    username: String,
    password: String,
    device_uuid: Option<String>,
  ) -> Self {
    let authentication =
      Authentication::from_credentials(username, password, device_uuid);

    Self {
      requires_2fa: false,
      login_response: None,
      request_manager: RequestManager::new(authentication.clone()),
      authentication,
    }
  }

  /// Initializes a session using the given credentials.
  /// `initialize` only performs the first step which is
  /// authenticating but does NOT return accounts nor 2FA challenge.
  ///
  /// See `finalize` for retrieving accounts.
  pub async fn initialize(&self) -> Result<(), Error> {
    Ok(())
  }

  pub fn finalize(&self) -> Result<Vec<Account>, Error> {
    if let Some(login_response) = &self.login_response {
      Ok(login_response.accounts.clone())
    } else {
      Err(Error::WrongLoginState())
    }
  }
}
