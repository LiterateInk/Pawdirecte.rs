use crate::api::Authentication;

#[derive(Debug, Clone)]
pub struct RequestManager {
  authentication: Authentication,
}

impl RequestManager {
  pub fn new(authentication: Authentication) -> Self {
    Self { authentication }
  }
}
