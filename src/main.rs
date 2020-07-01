use google_authenticator::GoogleAuthenticator;
use std::{env, error::Error};

const ENV_KEY: &str = "GA_SECRET";

fn main() -> Result<(), Box<dyn Error>> {
  let maybe_secret = env::var(ENV_KEY);
  match maybe_secret {
    Ok(s) => {
      let c = GoogleAuthenticator::new().get_code(&s, 0)?;
      println!("{}", c);
      std::process::exit(0)
    }
    Err(_) => {
      eprintln!("Missing {} environment variable!", ENV_KEY);
      std::process::exit(1)
    }
  }
}
