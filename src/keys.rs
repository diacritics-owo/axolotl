use age::{secrecy::Secret, Decryptor, Encryptor};
use base64::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
  fs,
  io::{Read, Write},
};

use crate::constants;

pub fn initialize() {
  if !constants::GLOBAL.as_path().exists() {
    fs::create_dir(constants::GLOBAL.as_path()).unwrap();
  }

  if !constants::KEYS.as_path().exists() {
    write(Keys {
      encrypted: false,
      modrinth: None,
      github: None,
    });
  }
}

pub fn read() -> Keys {
  let keys: Keys = toml::from_str(
    fs::read_to_string(constants::KEYS.as_path())
      .unwrap()
      .as_str(),
  )
  .unwrap();

  if keys.encrypted {}

  return keys;
}

pub fn write(keys: Keys) {
  fs::write(constants::KEYS.as_path(), toml::to_string(&keys).unwrap()).unwrap();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Keys {
  pub encrypted: bool,
  pub modrinth: Option<String>,
  pub github: Option<String>,
}

impl Keys {
  pub fn encrypt(plaintext: String, key: String) -> String {
    let encryptor = Encryptor::with_user_passphrase(Secret::new(key));

    let mut encrypted = vec![];
    let mut writer = encryptor.wrap_output(&mut encrypted).unwrap();
    writer.write_all(plaintext.as_bytes()).unwrap();
    writer.finish().unwrap();

    BASE64_STANDARD.encode(encrypted)
  }

  pub fn decrypt(encrypted: String, key: String) -> String {
    let decryptor = BASE64_STANDARD.decode(encrypted).unwrap();
    let decryptor = match Decryptor::new(&*decryptor).unwrap() {
      Decryptor::Passphrase(d) => d,
      _ => unreachable!(),
    };

    let mut decrypted = vec![];
    let mut reader = decryptor.decrypt(&Secret::new(key), None).unwrap();
    reader.read_to_end(&mut decrypted).unwrap();

    String::from_utf8(decrypted).unwrap()
  }

  pub fn encrypted(&self, key: String) -> Self {
    Keys {
      encrypted: true,
      modrinth: self
        .modrinth
        .clone()
        .map(|token| Self::encrypt(token, key.clone())),
      github: self.github.clone().map(|token| Self::encrypt(token, key)),
    }
  }

  pub fn decrypted(&self, key: String) -> Self {
    Keys {
      encrypted: false,
      modrinth: self
        .modrinth
        .clone()
        .map(|token| Self::decrypt(token, key.clone())),
      github: self.github.clone().map(|token| Self::decrypt(token, key)),
    }
  }

  pub fn to_string(&self) -> String {
    return format!(
      "Modrinth: {}\nGitHub: {}",
      self.modrinth.clone().unwrap_or("none".to_string()),
      self.github.clone().unwrap_or("none".to_string())
    );
  }
}
