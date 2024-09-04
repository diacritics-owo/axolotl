mod constants;
mod error;
mod keys;
mod util;

use clap::{Parser, Subcommand};
use inquire::Confirm;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Arguments {
  #[command(subcommand)]
  command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
  /// Manage keys and access tokens
  #[clap(alias = "k")]
  Keys {
    #[command(subcommand)]
    command: KeyCommands,
  },
}

#[derive(Subcommand, Debug)]
enum KeyCommands {
  #[clap(alias = "s")]
  Show,
  #[clap(alias = "e")]
  Encryption {
    #[command(subcommand)]
    command: Option<EncryptionCommands>,
  },
}

#[derive(Subcommand, Debug)]
enum EncryptionCommands {
  #[clap(alias = "e")]
  Enable,
  #[clap(alias = "d")]
  Disable,
}

fn main() -> Result<(), error::DeepslateError> {
  keys::initialize()?;

  let arguments = Arguments::parse();

  match arguments.command {
    Commands::Keys { command } => {
      let keys = keys::read_raw()?;

      match command {
        KeyCommands::Show => {
          if Confirm::new("Are you sure?")
            .with_default(false)
            .with_help_message("Your keys will be printed to stdout in plaintext form")
            .prompt()?
          {
            println!("{}", util::get_keys()?.to_string());
          }
        }
        KeyCommands::Encryption { command } => match command {
          Some(EncryptionCommands::Enable) => {
            if keys.encrypted {
              println!("Your keys are already encrypted - disable and re-enable encryption to change the key")
            } else {
              keys::write(keys.encrypted(util::read_key()?)?)?
            }
          }
          Some(EncryptionCommands::Disable) => {
            if !keys.encrypted {
              println!("Your keys have not been encrypted")
            } else {
              keys::write(keys.decrypted(util::read_key()?)?)?
            }
          }
          None => println!(
            "Encryption is {}",
            if keys.encrypted {
              "enabled"
            } else {
              "disabled"
            }
          ),
        },
      }
    }
  }

  Ok(())
}
