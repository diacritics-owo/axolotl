mod constants;
mod error;
mod keys;
mod util;

use clap::{Parser, Subcommand, ValueEnum};
use inquire::{Confirm, Text};
use util::get_keys;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Arguments {
  #[command(subcommand)]
  command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
  /// Manage keys and access tokens
  #[clap(visible_alias = "k")]
  Keys {
    #[command(subcommand)]
    command: KeyCommands,
  },
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
  keys::initialize()?;

  let arguments = Arguments::parse();

  match arguments.command {
    Commands::Keys { command } => {
      let raw = keys::read_raw()?;

      match command {
        KeyCommands::Print => {
          if Confirm::new("Are you sure?")
            .with_default(false)
            .with_help_message("Your keys will be printed to stdout in plaintext form")
            .prompt()?
          {
            println!("{}", util::get_keys()?.0.to_string());
          }
        }
        KeyCommands::Encryption { command } => match command {
          Some(EncryptionCommands::Enable) => {
            if raw.encrypted {
              println!("Your keys are already encrypted - disable and re-enable encryption to change the key")
            } else {
              keys::write(raw.encrypted(util::read_key_confirmation(true)?)?)?
            }
          }
          Some(EncryptionCommands::Disable) => {
            if !raw.encrypted {
              println!("Your keys have not been encrypted")
            } else {
              keys::write(raw.decrypted(util::read_key()?)?)?
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

          keys::write(keys)?;
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

          keys::write(keys)?;
        }
      }
    }
  }

  Ok(())
}
