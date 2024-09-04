use thiserror::Error;

#[derive(Error, Debug)]
pub enum DeepslateError {
  #[error("error while reading user input")]
  InputError(#[from] inquire::InquireError),

  #[error("io error")]
  IoError(#[from] std::io::Error),

  #[error("could not serialize toml")]
  SerializationError(#[from] toml::ser::Error),

  #[error("could not deserialize toml")]
  DeserializationError(#[from] toml::de::Error),

  #[error("encryption failed")]
  EncryptionError(#[from] age::EncryptError),

  #[error("decryption failed")]
  DecryptionError(#[from] age::DecryptError),

  #[error("failed to create string from bytes")]
  Utf8Error(#[from] std::string::FromUtf8Error),

  #[error("failed to decode base64")]
  Base64DecodeError(#[from] base64::DecodeError),

  #[error("github api request failed")]
  GithubError(#[from] octocrab::Error),

  #[error("{0}")]
  Error(String),
}
