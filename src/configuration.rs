use crate::{constants, error, file::ToRead};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
  pub artifact: Artifact,
  pub changelog: Option<Changelog>,
  pub modrinth: Option<Modrinth>,
  pub github: Option<GitHub>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Artifact {
  pub folder: PathBuf,
  pub pattern: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Modrinth {
  pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitHub {
  pub repo: (String, String),
  #[serde(default = "default_draft")]
  pub draft: bool,
}

fn default_draft() -> bool {
  false
}

impl Default for Configuration {
  fn default() -> Self {
    Self {
      artifact: Artifact {
        folder: PathBuf::from("build/libs"),
        pattern: "mod-#.jar".to_string(),
      },
      changelog: Some(Changelog::Editor),
      modrinth: Some(Modrinth {
        id: "modrinth project id".to_string(),
      }),
      github: Some(GitHub {
        repo: ("user".to_string(), "repo".to_string()),
        draft: true,
      }),
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
    Ok(PathBuf::from(constants::CONFIGURATION).exists())
  }

  pub fn read() -> Result<Self, error::DeepslateError> {
    let file = ToRead::new(constants::CONFIGURATION)?;
    Ok(toml::from_str(&file.read_to_string()?)?)
  }

  pub fn write(configuration: Self) -> Result<(), error::DeepslateError> {
    Ok(fs::write(
      constants::CONFIGURATION,
      toml::to_string(&configuration)?,
    )?)
  }
}
