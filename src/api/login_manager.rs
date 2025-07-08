use std::{collections::HashMap, str::FromStr};

use crate::{
  api::{
    API_URL, API_VERSION, Authentication, Error, RequestManager, USER_AGENT,
  },
  definitions::{
    models::Account, requests::LoginRequest, responses::LoginResponse,
  },
};

use cookie_parser::{CookiePair, SetCookie, parse_set_cookie};
use reqwest::{
  Client,
  header::{self, HeaderName, SET_COOKIE},
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
    let mut gtk: Option<String> = None;
    let mut cookies: Vec<CookiePair> = vec![];

    let url = format!("{API_URL}/v3/login.awp?gtk=1&v={API_VERSION}");

    let client = Client::new();
    let response = client
      .get(&url)
      .header(header::USER_AGENT, USER_AGENT)
      .send()
      .await?;

    for cookie in response.headers().get_all(SET_COOKIE).iter() {
      let cookie = parse_set_cookie(cookie.to_str().unwrap())?;

      if cookie.pair.name == "GTK" {
        gtk = Some(cookie.pair.value.clone());
      }

      cookies.push(cookie.pair);
    }

    if let Some(gtk) = gtk {
      let cookies = cookies
        .iter()
        .map(|pair| format!("{}={}", pair.name, pair.value))
        .collect::<Vec<String>>()
        .join("; ");

      let mut params = HashMap::new();

      // We're not already authenticated, let's run the initial procedure.
      if self.authentication.access_token.is_none() {
        let auth = self.authentication.clone();

        let json = LoginRequest {
          device_uuid: auth.device_uuid,
          is_reauth: false,
          password: auth.password.trim().into(),
          remember_me: Some(true),
          username: auth.username,
          access_token: None,
          account_type: None,
          double_auth: None,
        };

        let json = serde_json::to_string(&json)?;
        params.insert("data", json);
      }

      let url = format!("{API_URL}/v3/login.awp?v={API_VERSION}");
      let response = client
        .post(&url)
        .header("X-GTK", &gtk)
        .header(header::COOKIE, &cookies)
        .header(header::USER_AGENT, USER_AGENT)
        .form(&params)
        .send()
        .await?;

      let text = response.text().await?;
      println!("{text}");

      Ok(())
    } else {
      Err(Error::CookieGtkNotFound())
    }
  }

  pub fn finalize(&self) -> Result<Vec<Account>, Error> {
    if let Some(login_response) = &self.login_response {
      Ok(login_response.accounts.clone())
    } else {
      Err(Error::WrongLoginState())
    }
  }
}
