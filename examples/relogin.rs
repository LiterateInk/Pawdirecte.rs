use std::env::var;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  dotenvy::from_path_override("examples/.env")?;

  let mut login = pawdirecte::api::LoginManager::from_access_token(
    var("USERNAME")?,
    var("ACCESS_TOKEN")?,
    var("KIND")?,
    var("DEVICE_UUID")?,
  );

  // Attempt a login, you might not have to handle double auth
  // since using access token will bypass this step for you.
  login.request().await?;

  // You can now use the accounts,
  // please see the other examples to see what you can do.
  let accounts = login.accounts()?;

  // --------------------------------------------------------------------------
  // A new access token has been generated while doing this operation,
  // make sure to save this!
  let account = accounts.first().unwrap();
  println!("ACCESS_TOKEN={}", account.access_token);

  Ok(())
}
