use modrinth_api::apis::{
  projects_api::CheckProjectValidityError, versions_api::CreateVersionError,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DeepslateError {
  #[error("error while reading user input: {0:#?}")]
  InputError(#[from] inquire::InquireError),

  #[error("io error: {0:#?}")]
  IoError(#[from] std::io::Error),

  #[error("could not serialize toml: {0:#?}")]
  TomlSerializationError(#[from] toml::ser::Error),

  #[error("could not serialize/deserialize json: {0:#?}")]
  JsonError(#[from] serde_json::Error),

  #[error("could not deserialize toml: {0:#?}")]
  TomlDeserializationError(#[from] toml::de::Error),

  #[error("encryption failed: {0:#?}")]
  EncryptionError(#[from] age::EncryptError),

  #[error("decryption failed: {0:#?}")]
  DecryptionError(#[from] age::DecryptError),

  #[error("failed to create string from bytes: {0:#?}")]
  Utf8Error(#[from] std::string::FromUtf8Error),

  #[error("failed to decode base64: {0:#?}")]
  Base64DecodeError(#[from] base64::DecodeError),

  #[error("github api request failed: {0:#?}")]
  GithubError(#[from] octocrab::Error),

  #[error("modrinth project validity check failed: {0:#?}")]
  ModrinthProjectValidityError(#[from] modrinth_api::apis::Error<CheckProjectValidityError>),

  #[error("modrinth version creation failed: {0:#?}")]
  ModrinthCreateVersionError(#[from] modrinth_api::apis::Error<CreateVersionError>),

  #[error("modrinth api request failed: {0:#?}")]
  ModrinthError(#[from] modrinth_api::apis::Error<()>),

  #[error("invalid header value: {0:#?}")]
  InvalidHeaderValueError(#[from] reqwest::header::InvalidHeaderValue),

  #[error("reqwest error: {0:#?}")]
  ReqwestError(#[from] reqwest::Error),

  #[error("{0:#?}")]
  Error(String),
}
