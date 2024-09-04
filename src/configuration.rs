use crate::{constants, error};
use serde::{Deserialize, Serialize};
use std::{env, fs};

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
  pub changelog: Option<Changelog>,
  pub modrinth: Option<String>,
  pub github: Option<(String, String)>,
}

impl Default for Configuration {
  fn default() -> Self {
    Self {
      changelog: Some(Changelog::Editor),
      modrinth: Some("modrinth project id".to_string()),
      github: Some(("user".to_string(), "repo".to_string())),
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Changelog {
  File { file: String },
  Editor,
}

impl Configuration {
  pub fn exists() -> Result<bool, error::DeepslateError> {
    Ok(env::current_dir()?.join(constants::CONFIGURATION).exists())
  }

  pub fn read() -> Result<Self, error::DeepslateError> {
    Ok(toml::from_str(&fs::read_to_string(
      env::current_dir()?.join(constants::CONFIGURATION),
    )?)?)
  }

  pub fn write(configuration: Self) -> Result<(), error::DeepslateError> {
    Ok(fs::write(
      constants::CONFIGURATION,
      toml::to_string(&configuration)?,
    )?)
  }
}
