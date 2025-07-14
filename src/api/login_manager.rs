use crate::{
  api::{Authentication, Error, RequestBuilder, RequestManager},
  definitions::{
    models::Account,
    requests::{DoubleAuthSolveRequest, EmptyRequest, LoginRequest},
    responses::{
      DoubleAuthChallengeResponse, DoubleAuthSolveResponse, LoginResponse,
    },
  },
};

use cookie_parser::{CookiePair, parse_set_cookie};
use http::Method;
use reqwest::header::{self, SET_COOKIE};
use std::{
  sync::{Arc, Mutex},
  vec,
};

#[derive(Debug)]
pub struct LoginManager {
  pub requires_2fa: bool,
  login_response: Option<LoginResponse>,
  request_manager: RequestManager,
  pub authentication: Arc<Mutex<Authentication>>,
  double_auth: Option<DoubleAuthSolveResponse>,
}

impl LoginManager {
  pub fn from_credentials(
    username: String,
    password: String,
    device_uuid: Option<String>,
  ) -> Self {
    let authentication = Arc::new(Mutex::new(
      Authentication::from_credentials(username, password, device_uuid),
    ));

    Self {
      requires_2fa: false,
      login_response: None,
      request_manager: RequestManager::new(authentication.clone()),
      authentication,
      double_auth: None,
    }
  }

  /// Make a login request, this will define accounts and 2FA variable.
  pub async fn request(&mut self) -> Result<(), Error> {
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
      let auth = self.authentication.lock().unwrap().clone();
      let form = if auth.access_token.is_none() {
        let double_auth = if self.double_auth.is_none() {
          None
        } else {
          Some(vec![self.double_auth.clone().unwrap()])
        };

        LoginRequest {
          device_uuid: auth.device_uuid,
          is_reauth: false,
          password: auth.password.trim().into(),
          remember_me: Some(true),
          username: auth.username,
          access_token: None,
          account_type: None,
          double_auth,
        }
      }
      // 7.2.1. we're already authenticated, re-use the access token.
      else {
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

      match json.code {
        505 => Err(Error::BadCredentials()),
        517 => Err(Error::InvalidVersion()),
        535 => Err(Error::EstablishmentUnavailable()),
        _ => {
          // 11. check if 2fa is required.
          self.requires_2fa = json.code == 250;

          // 12. assign the login response, for later usage.
          self.login_response = Some(json.data);

          Ok(())
        }
      }
    } else {
      Err(Error::CookieGtkNotFound())
    }
  }

  pub async fn get_2fa_challenge(
    &mut self,
  ) -> Result<DoubleAuthChallengeResponse, Error> {
    let auth = self.authentication.lock().unwrap().clone();

    let request = RequestBuilder::new(
      Method::POST,
      "/v3/connexion/doubleauth.awp?verbe=get",
    )?
    .append_version()
    .set_token(auth.token.unwrap())?
    .set_form(EmptyRequest {})
    .build()?;

    let (json, _) = self
      .request_manager
      .send::<DoubleAuthChallengeResponse>(request)
      .await?;

    Ok(json.unwrap().data)
  }

  pub async fn solve_2fa_challenge(
    &mut self,
    answer: String,
  ) -> Result<(), Error> {
    let auth = self.authentication.lock().unwrap().clone();

    let request = RequestBuilder::new(
      Method::POST,
      "/v3/connexion/doubleauth.awp?verbe=post",
    )?
    .append_version()
    .set_token(auth.token.unwrap())?
    .set_form(DoubleAuthSolveRequest { answer })
    .build()?;

    let (json, _) = self
      .request_manager
      .send::<DoubleAuthSolveResponse>(request)
      .await?;

    let json = json.unwrap().data;
    self.double_auth = Some(json);

    Ok(())
  }

  pub fn accounts(&self) -> Result<Vec<Account>, Error> {
    if let Some(login_response) = &self.login_response {
      Ok(login_response.accounts.clone())
    } else {
      Err(Error::WrongLoginState())
    }
  }
}
