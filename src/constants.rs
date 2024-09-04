use dirs::config_dir;
use lazy_static::lazy_static;
use std::path::PathBuf;

pub static CONFIGURATION: &str = "deepslate.toml";
pub static VERSION_REPLACE: &str = "#";

lazy_static! {
  pub static ref GLOBAL: PathBuf = config_dir().unwrap().join("deepslate");
  pub static ref KEYS: PathBuf = GLOBAL.join("keys.toml");
}
