#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("unauthenticated session")]
  WrongLoginState(),
  #[error("identifiers or tokens are incorrect")]
  BadCredentials(),
  #[error("a newer version of EcoleDirecte is available")]
  InvalidVersion(),
  #[error("establishment is closed, wait until it reopens")]
  EstablishmentUnavailable(),
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
