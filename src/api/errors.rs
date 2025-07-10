#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("unauthenticated session")]
  WrongLoginState(),
  #[error("GTK cookie not found, is EcoleDirecte up?")]
  CookieGtkNotFound(),
  #[error(transparent)]
  Network(#[from] reqwest::Error),
  #[error(transparent)]
  HeaderValue(#[from] reqwest::header::InvalidHeaderValue),
  #[error(transparent)]
  Cookie(#[from] cookie_parser::CookieParseError),
  #[error(transparent)]
  JsonEncode(#[from] serde_json::Error),
  #[error(transparent)]
  Url(#[from] url::ParseError),
  #[error(transparent)]
  FormEncode(#[from] serde_urlencoded::ser::Error),
}
