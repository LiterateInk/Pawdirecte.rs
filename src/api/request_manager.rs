use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};

use reqwest::{
  Client, Request,
  header::{self, HeaderMap},
};
use url::Url;

use crate::{
  api::{API_URL, API_VERSION, Authentication, Error, USER_AGENT},
  definitions::api::APIResponseWrap,
};

#[derive(Debug, Clone)]
pub struct RequestManager {
  authentication: Arc<Mutex<Authentication>>,
}

impl RequestManager {
  pub fn new(authentication: Arc<Mutex<Authentication>>) -> Self {
    Self { authentication }
  }

  pub async fn send<T: serde::de::DeserializeOwned + Debug>(
    &mut self,
    request: Request,
  ) -> Result<(Option<APIResponseWrap<T>>, HeaderMap), Error> {
    let client = Client::new();

    let response = client.execute(request).await?;
    let headers = response.headers().clone();
    let json = response.json::<APIResponseWrap<T>>().await;

    if let Ok(json) = json {
      if let Some(token) = json.token.as_ref() {
        let mut auth = self.authentication.lock().unwrap();
        auth.token = Some(token.clone());
      }

      Ok((Some(json), headers))
    } else {
      Ok((None, headers))
    }
  }
}

pub struct RequestBuilder<T: serde::Serialize> {
  method: http::Method,
  headers: HeaderMap,
  form: Option<T>,
  url: Url,
}

impl<T: serde::Serialize> RequestBuilder<T> {
  pub fn new(method: http::Method, path: &str) -> Result<Self, Error> {
    let url = Url::parse(format!("{API_URL}{path}").as_ref())?;

    let mut headers = HeaderMap::new();
    headers.insert(header::USER_AGENT, USER_AGENT.parse()?);

    Ok(Self {
      method,
      headers,
      form: None,
      url,
    })
  }

  pub fn append_version(mut self) -> Self {
    self.url.query_pairs_mut().append_pair("v", API_VERSION);
    self
  }

  pub fn set_token(mut self, token: String) -> Result<Self, Error> {
    self.headers.insert("X-Token", token.parse()?);
    Ok(self)
  }

  pub fn set_form(mut self, data: T) -> Self {
    self.form = Some(data);
    self
  }

  pub fn build(self) -> Result<Request, Error> {
    let mut request = Request::new(self.method, self.url);
    *request.headers_mut() = self.headers;

    if let Some(form) = self.form {
      let json = serde_json::to_string(&form)?;

      let mut params = HashMap::new();
      params.insert("data", json);

      let body = serde_urlencoded::to_string(&params)?;
      *request.body_mut() = Some(body.into());

      request.headers_mut().insert(
        header::CONTENT_TYPE,
        "application/x-www-form-urlencoded".parse()?,
      );
    }

    Ok(request)
  }
}
