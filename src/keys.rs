use crate::{constants, error, file::ToRead};
use age::{secrecy::Secret, Decryptor, Encryptor};
use base64::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
  fs,
  io::{Read, Write},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Keys {
  pub encrypted: bool,
  pub modrinth: Option<String>,
  pub github: Option<String>,
}

impl Keys {
  pub fn initialize() -> Result<(), error::AxolotlError> {
    if !constants::GLOBAL.as_path().exists() {
      fs::create_dir(constants::GLOBAL.as_path())?;
    }

    if !constants::KEYS.as_path().exists() {
      Keys::write(Keys::default())?;
    }

    Ok(())
  }

  pub fn read_raw() -> Result<Keys, error::AxolotlError> {
    let file = ToRead::new(constants::KEYS.as_path())?;
    return Ok(toml::from_str(file.read_to_string()?.as_str())?);
  }

  pub fn write(keys: Keys) -> Result<(), error::AxolotlError> {
    Ok(fs::write(
      constants::KEYS.as_path(),
      toml::to_string(&keys)?,
    )?)
  }

  pub fn encrypt(plaintext: String, key: String) -> Result<String, error::AxolotlError> {
    let encryptor = Encryptor::with_user_passphrase(Secret::new(key));

    let mut encrypted = vec![];
    let mut writer = encryptor.wrap_output(&mut encrypted)?;
    writer.write_all(plaintext.as_bytes())?;
    writer.finish()?;

    Ok(BASE64_STANDARD.encode(encrypted))
  }

  pub fn decrypt(encrypted: String, key: String) -> Result<String, error::AxolotlError> {
    let decryptor = BASE64_STANDARD.decode(encrypted)?;
    let decryptor = match Decryptor::new(&*decryptor)? {
      Decryptor::Passphrase(d) => d,
      _ => unreachable!(),
    };

    let mut decrypted = vec![];
    let mut reader = decryptor.decrypt(&Secret::new(key), None)?;
    reader.read_to_end(&mut decrypted)?;

    Ok(String::from_utf8(decrypted)?)
  }

  pub fn encrypted(&self, key: String) -> Result<Keys, error::AxolotlError> {
    Ok(Keys {
      encrypted: true,
      modrinth: match self.modrinth.clone() {
        Some(token) => Some(Self::encrypt(token, key.clone())?),
        None => None,
      },
      github: match self.github.clone() {
        Some(token) => Some(Self::encrypt(token, key)?),
        None => None,
      },
    })
  }

  pub fn decrypted(&self, key: String) -> Result<Keys, error::AxolotlError> {
    Ok(Keys {
      encrypted: false,
      modrinth: match self.modrinth.clone() {
        Some(token) => Some(Self::decrypt(token, key.clone())?),
        None => None,
      },
      github: match self.github.clone() {
        Some(token) => Some(Self::decrypt(token, key)?),
        None => None,
      },
    })
  }
}

impl Default for Keys {
  fn default() -> Self {
    Keys {
      encrypted: false,
      modrinth: None,
      github: None,
    }
  }
}
