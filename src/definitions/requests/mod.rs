use serde::Serialize;
automod::dir!("src/definitions/requests");

pub use double_auth::*;
pub use login::*;

#[derive(Debug, Serialize)]
pub struct EmptyRequest {}
