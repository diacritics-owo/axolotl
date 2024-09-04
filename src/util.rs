use std::path::Path;

use crate::{error, keys::Keys};
use inquire::{Password, PasswordDisplayMode};

pub fn assert_exists<P: AsRef<Path>>(path: P) -> Result<(), error::DeepslateError> {
  if path.as_ref().exists() {
    Ok(())
  } else {
    Err(error::DeepslateError::Error(format!(
      "Could not find the file {}",
      path.as_ref().to_str().unwrap_or("<failed to get path>")
    )))
  }
}

pub fn read_key() -> Result<String, error::DeepslateError> {
  read_key_confirmation(false)
}

pub fn read_key_confirmation(confirm: bool) -> Result<String, error::DeepslateError> {
  let mut password = Password::new("Enter your passphrase").with_display_mode(PasswordDisplayMode::Masked);

  if !confirm {
    password.enable_confirmation = confirm;
  }

  Ok(password.prompt()?)
}

pub fn get_keys() -> Result<(Keys, Option<String>), error::DeepslateError> {
  let keys = Keys::read_raw()?;

  if keys.encrypted {
    let key = read_key()?;
    let mut keys = keys.decrypted(key.clone())?;
    keys.encrypted = true;

    return Ok((keys, Some(key)));
  }

  return Ok((keys, None));
}
