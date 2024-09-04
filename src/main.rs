mod configuration;
mod constants;
mod error;
mod keys;
mod util;

use std::{env::current_dir, fs};

use clap::{Parser, Subcommand, ValueEnum};
use configuration::{Changelog, Configuration};
use inquire::{Confirm, Editor, Text};
use keys::Keys;
use util::get_keys;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Arguments {
  #[command(subcommand)]
  command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
  /// Manage mods
  #[clap(visible_alias = "m")]
  Mod {
    #[command(subcommand)]
    command: ModCommands,
  },

  /// Manage keys and access tokens
  #[clap(visible_alias = "k")]
  Keys {
    #[command(subcommand)]
    command: KeyCommands,
  },
}

#[derive(Subcommand, Debug)]
enum ModCommands {
  /// Create a configuration file
  #[clap(visible_alias = "i")]
  Init,

  /// Publish the mod
  #[clap(visible_alias = "p")]
  Publish,
}

#[derive(Subcommand, Debug)]
enum KeyCommands {
  /// Print plaintext keys to stdout
  #[clap(visible_alias = "p")]
  Print,

  /// Manage key encryption
  #[clap(visible_alias = "e")]
  Encryption {
    #[command(subcommand)]
    command: Option<EncryptionCommands>,
  },

  /// Set a key
  #[clap(visible_alias = "s")]
  Set {
    #[arg(value_enum)]
    distributor: Distributor,
  },

  /// Remove a key
  #[clap(visible_alias = "r")]
  Remove {
    #[arg(value_enum)]
    distributor: Distributor,
  },
}

#[derive(Subcommand, Debug)]
enum EncryptionCommands {
  /// Enable encryption
  #[clap(visible_alias = "e")]
  Enable,

  /// Disable encryption
  #[clap(visible_alias = "d")]
  Disable,
}

#[derive(Debug, Clone, ValueEnum)]
#[clap(rename_all = "lower")]
enum Distributor {
  /// [aliases: m]
  #[clap(alias = "m")]
  Modrinth,

  /// [aliases: g]
  #[clap(alias = "g")]
  GitHub,
}

fn main() -> Result<(), error::DeepslateError> {
  Keys::initialize()?;

  let arguments = Arguments::parse();

  match arguments.command {
    Commands::Mod { command } => match command {
      ModCommands::Init => {
        if Configuration::exists()? {
          println!(
            "Found pre-existing configuration file {}",
            constants::CONFIGURATION
          );
        } else {
          Configuration::write(Configuration::default())?;
        }
      }
      // TODO
      ModCommands::Publish => {
        let configuration = Configuration::read()?;
        let changelog = match configuration.changelog {
          Some(changelog) => match changelog {
            Changelog::File { file } => Some(fs::read_to_string(current_dir()?.join(file))?),
            Changelog::Editor => Some(Editor::new("Write the changelog").prompt()?),
          },
          None => None,
        };

        if let Some(id) = configuration.modrinth {}

        if let Some((user, repo)) = configuration.github {}
      }
    },
    Commands::Keys { command } => {
      let raw = Keys::read_raw()?;

      match command {
        KeyCommands::Print => {
          if Confirm::new("Are you sure?")
            .with_default(false)
            .with_help_message("Your keys will be printed to stdout in plaintext form")
            .prompt()?
          {
            println!("{:#?}", util::get_keys()?.0);
          }
        }
        KeyCommands::Encryption { command } => match command {
          Some(EncryptionCommands::Enable) => {
            if raw.encrypted {
              println!("Your keys are already encrypted - disable and re-enable encryption to change the key")
            } else {
              Keys::write(raw.encrypted(util::read_key_confirmation(true)?)?)?
            }
          }
          Some(EncryptionCommands::Disable) => {
            if !raw.encrypted {
              println!("Your keys have not been encrypted")
            } else {
              Keys::write(raw.decrypted(util::read_key()?)?)?
            }
          }
          None => println!(
            "Encryption is {}",
            if raw.encrypted { "enabled" } else { "disabled" }
          ),
        },
        KeyCommands::Set { distributor } => {
          let (mut keys, key) = get_keys()?;
          let value = Text::new("Enter the new key").prompt()?;

          match distributor {
            Distributor::Modrinth => keys.modrinth = Some(value),
            Distributor::GitHub => keys.github = Some(value),
          }

          if let Some(key) = key {
            keys = keys.encrypted(key)?;
          }

          Keys::write(keys)?;
        }
        KeyCommands::Remove { distributor } => {
          let (mut keys, key) = get_keys()?;

          match distributor {
            Distributor::Modrinth => keys.modrinth = None,
            Distributor::GitHub => keys.github = None,
          }

          if let Some(key) = key {
            keys = keys.encrypted(key)?;
          }

          Keys::write(keys)?;
        }
      }
    }
  }

  Ok(())
}
