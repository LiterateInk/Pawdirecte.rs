use inquire::Select;
use std::env::var;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  dotenvy::from_path_override("examples/.env")?;

  let mut login = pawdirecte::api::LoginManager::from_credentials(
    var("USERNAME")?,
    var("PASSWORD")?,
    None,
  );

  // Attempt a login.
  login.request().await?;

  // 2FA is required to log in, you should solve the challenge.
  if login.requires_2fa {
    // Retrieve the challenge, contains a question with possible answers.
    let challenge = login.get_2fa_challenge().await?;

    // Display all the possible answers and reply.
    let answer =
      Select::new(&challenge.question, challenge.answers).prompt()?;

    // Send our answer to the server to validate.
    login.solve_2fa_challenge(answer).await?;

    // Run a new login attempt.
    login.request().await?;
  }

  // You can now use the accounts!
  let accounts = login.accounts()?;

  // We're logging them, please see the other examples to see what you can do.
  println!("{accounts:#?}");

  Ok(())
}
