use crate::{
  error,
  keys::{self, Keys},
};
use inquire::Text;

pub fn read_key() -> Result<String, error::DeepslateError> {
  read_string("key")
}

pub fn read_string(name: &str) -> Result<String, error::DeepslateError> {
  Ok(Text::new(format!("Enter your {}", name).as_str()).prompt()?)
}

pub fn get_keys() -> Result<Keys, error::DeepslateError> {
  let mut keys = keys::read_raw()?;

  if keys.encrypted {
    keys = keys.decrypted(read_key()?)?;
  }

  return Ok(keys);
}
