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
  pub game_versions: Vec<String>,
  pub loaders: Vec<String>,
}

// TODO: drafts
#[derive(Debug, Serialize, Deserialize)]
pub struct Modrinth {
  pub id: String,
  #[serde(default = "default_featured")]
  pub featured: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitHub {
  pub repo: (String, String),
  #[serde(default = "default_draft")]
  pub draft: bool,
}

fn default_draft() -> bool {
  true
}

fn default_featured() -> bool {
  true
}

impl Default for Configuration {
  fn default() -> Self {
    Self {
      artifact: Artifact {
        folder: PathBuf::from("build/libs"),
        pattern: "mod-#.jar".to_string(),
        game_versions: vec!["1.xx".to_string()],
        loaders: vec![
          "fabric".to_string(),
          "quilt".to_string(),
          "forge".to_string(),
          "neoforge".to_string(),
        ],
      },
      changelog: Some(Changelog::Editor),
      modrinth: Some(Modrinth {
        id: "modrinth project id".to_string(),
        featured: true,
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
  pub fn exists() -> Result<bool, error::AxolotlError> {
    Ok(PathBuf::from(constants::CONFIGURATION).exists())
  }

  pub fn read() -> Result<Self, error::AxolotlError> {
    let file = ToRead::new(constants::CONFIGURATION)?;
    Ok(toml::from_str(&file.read_to_string()?)?)
  }

  pub fn write(configuration: Self) -> Result<(), error::AxolotlError> {
    Ok(fs::write(
      constants::CONFIGURATION,
      toml::to_string(&configuration)?,
    )?)
  }
}
