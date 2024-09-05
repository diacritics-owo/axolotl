use tokio::fs::File;

use crate::error;
use std::{fs, io, path::Path};

pub struct ToRead<P: AsRef<Path>>(P);

impl<P: AsRef<Path> + Clone> ToRead<P> {
  pub fn new(path: P) -> Result<Self, error::DeepslateError> {
    if path.as_ref().exists() {
      Ok(Self(path))
    } else {
      Err(error::DeepslateError::Error(format!(
        "Could not find the file {}",
        path.as_ref().to_str().unwrap_or("<failed to get path>")
      )))
    }
  }

  pub fn read_to_string(&self) -> io::Result<String> {
    fs::read_to_string(self.0.clone())
  }

  pub async fn open(&self) -> io::Result<File> {
    File::open(self.0.clone()).await
  }
}
