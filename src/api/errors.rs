#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("unauthenticated session")]
  WrongLoginState(),
  #[error("GTK cookie not found, is EcoleDirecte up?")]
  CookieGtkNotFound(),
  #[error(transparent)]
  Network(#[from] reqwest::Error),
  #[error(transparent)]
  Cookie(#[from] cookie_parser::CookieParseError),
  #[error(transparent)]
  Json(#[from] serde_json::Error),
}
