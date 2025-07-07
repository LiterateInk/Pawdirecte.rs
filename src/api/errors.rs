#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("unauthenticated session")]
  WrongLoginState(),
}
