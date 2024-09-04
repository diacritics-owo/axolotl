use crate::keys::{self, Keys};
use inquire::Text;

pub fn read_key() -> String {
  read_string("key")
}

pub fn read_string(name: &str) -> String {
  Text::new(format!("Enter your {}", name).as_str())
    .prompt()
    .unwrap()
}

pub fn get_keys() -> Keys {
  let mut keys = keys::read();

  if keys.encrypted {
    keys = keys.decrypted(read_key());
  }

  return keys;
}
