use crate::{error, keys::Keys};
use inquire::{Password, PasswordDisplayMode};
use modrinth_api::models;
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VersionType {
  Release,
  Beta,
  Alpha,
}

impl Into<models::creatable_version::VersionType> for VersionType {
  fn into(self) -> models::creatable_version::VersionType {
    match self {
      Self::Release => models::creatable_version::VersionType::Release,
      Self::Beta => models::creatable_version::VersionType::Beta,
      Self::Alpha => models::creatable_version::VersionType::Alpha,
    }
  }
}

impl Display for VersionType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Self::Release => "Release",
        Self::Beta => "Beta",
        Self::Alpha => "Alpha",
      }
    )
  }
}

pub fn read_key() -> Result<String, error::AxolotlError> {
  read_key_confirmation(false)
}

pub fn read_key_confirmation(confirm: bool) -> Result<String, error::AxolotlError> {
  let mut password =
    Password::new("Enter your passphrase").with_display_mode(PasswordDisplayMode::Masked);

  if !confirm {
    password.enable_confirmation = confirm;
  }

  Ok(password.prompt()?)
}

pub fn get_keys() -> Result<(Keys, Option<String>), error::AxolotlError> {
  let keys = Keys::read_raw()?;

  if keys.encrypted {
    let key = read_key()?;
    let mut keys = keys.decrypted(key.clone())?;
    keys.encrypted = true;

    return Ok((keys, Some(key)));
  }

  return Ok((keys, None));
}
