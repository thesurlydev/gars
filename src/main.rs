extern crate clipboard;

use clipboard::ClipboardProvider;
use clipboard_ext::x11_fork::ClipboardContext;
use google_authenticator::GoogleAuthenticator;
use google_authenticator::GAError;

fn main() {
  let maybe_secret: Option<&'static str> = option_env!("GA_SECRET");
  match maybe_secret {
    Some(s) => {
      let auth = GoogleAuthenticator::new();
      let maybe_code: Result<String, GAError> = auth.get_code(s, 0);
      match maybe_code {
        Ok(c) => {
          let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
          let msg = c.clone();
          ctx.set_contents(c.to_owned()).unwrap();
          println!("{}", msg);
        },
        Err(e) => eprintln!("No Google Authenticator code returned: {:?}", e)
      }
    },
    None => eprintln!("Missing GA_SECRET environment variable!")
  }
}
