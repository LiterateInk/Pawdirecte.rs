use crate::{
  api::{Authentication, Error, RequestBuilder, RequestManager},
  definitions::{
    models::Account, requests::LoginRequest, responses::LoginResponse,
  },
};

use cookie_parser::{CookiePair, parse_set_cookie};
use http::Method;
use reqwest::header::{self, SET_COOKIE};

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
  pub async fn initialize(&mut self) -> Result<(), Error> {
    // 1. craft a request to grab GTK cookies for login.
    let request =
      RequestBuilder::<()>::new(Method::GET, "/v3/login.awp?gtk=1")?
        .append_version()
        .build()?;

    // 2. send the request and get the response.
    let (_, headers) = self.request_manager.send::<()>(request).await?;

    // 3. extract new cookies from "set-cookie" header.
    let cookies: Vec<CookiePair> = headers
      .get_all(SET_COOKIE)
      .iter()
      .map(|cookie_header| {
        parse_set_cookie(cookie_header.to_str().unwrap())
          .unwrap()
          .pair
      })
      .collect();

    // 4. find the "GTK" cookie within all the new cookies.
    let gtk = cookies
      .iter()
      .find(|pair| pair.name == "GTK")
      .map(|pair| pair.value.clone());

    // 5. make sure it exists.
    if let Some(gtk) = gtk {
      // 6. re-use the GWT (+extra) cookies for the authentication
      let cookies = cookies
        .iter()
        .map(|pair| format!("{}={}", pair.name, pair.value))
        .collect::<Vec<_>>()
        .join("; ");

      // 7. build the form data to authenticate.
      // ---------------------------------------
      // 7.1.1. we're not already authenticated, let's run the initial procedure.
      let form = if self.authentication.access_token.is_none() {
        let auth = self.authentication.clone();

        LoginRequest {
          device_uuid: auth.device_uuid,
          is_reauth: false,
          password: auth.password.trim().into(),
          remember_me: Some(true),
          username: auth.username,
          access_token: None,
          account_type: None,
          double_auth: None,
        }
      }
      // 7.2.1. we're already authenticated, re-use the access token.
      else {
        let auth = self.authentication.clone();
        LoginRequest {
          device_uuid: auth.device_uuid,
          is_reauth: true,
          password: "???".into(),
          remember_me: None,
          access_token: auth.access_token,
          double_auth: None,
          username: auth.username,
          account_type: None, // TODO
        }
      };

      // 8. craft a request to login with GTK cookies, using the previous payload.
      let mut request = RequestBuilder::new(Method::POST, "/v3/login.awp")?
        .append_version()
        .set_form(form)
        .build()?;

      // 9. append the GTK cookies to the crafted request.
      let headers = request.headers_mut();
      _ = headers.insert("X-GTK", gtk.parse()?);
      _ = headers.insert(header::COOKIE, cookies.parse()?);

      // 10. send the request and get the response.
      let (json, _) =
        self.request_manager.send::<LoginResponse>(request).await?;

      // 10. we're sure that JSON is given to user.
      let json = json.unwrap();

      // 11. check if 2fa is required.
      self.requires_2fa = json.code == 250;

      // 12. assign the login response, for later usage.
      self.login_response = Some(json.data);

      Ok(())
    } else {
      Err(Error::CookieGtkNotFound())
    }
  }

  pub async fn get_2fa_challenge(&self) -> Result<(), Error> {
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
