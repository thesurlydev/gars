use std::error::Error;
use google_authenticator::GoogleAuthenticator;

fn main() -> Result<(), Box<dyn Error>> {
  let maybe_secret: Option<&'static str> = option_env!("GA_SECRET");
  match maybe_secret {
    Some(s) => println!("{}", GoogleAuthenticator::new().get_code(s, 0)?),
    None => eprintln!("Missing GA_SECRET environment variable!")
  }
  Ok(())
}
