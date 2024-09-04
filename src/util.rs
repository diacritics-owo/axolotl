use crate::{
  error,
  keys::{self, Keys},
};
use inquire::{Password, PasswordDisplayMode};

pub fn read_key() -> Result<String, error::DeepslateError> {
  read_key_confirmation(false)
}

pub fn read_key_confirmation(confirm: bool) -> Result<String, error::DeepslateError> {
  let mut password = Password::new("Enter your key").with_display_mode(PasswordDisplayMode::Masked);

  if !confirm {
    password.enable_confirmation = confirm;
  }

  Ok(password.prompt()?)
}

pub fn get_keys() -> Result<(Keys, Option<String>), error::DeepslateError> {
  let keys = keys::read_raw()?;

  if keys.encrypted {
    let key = read_key()?;
    return Ok((keys.decrypted(key.clone())?, Some(key)));
  }

  return Ok((keys, None));
}
