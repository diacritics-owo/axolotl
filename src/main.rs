extern crate pretty_env_logger;

#[macro_use]
extern crate log;

mod configuration;
mod constants;
mod error;
mod file;
mod keys;
mod modrinth;
mod util;

use clap::{Parser, Subcommand, ValueEnum};
use configuration::{Changelog, Configuration, GitHub, Modrinth, ModrinthDependency};
use file::ToRead;
use inquire::{Confirm, Editor, Select, Text};
use keys::Keys;
use modrinth_api::{
  apis,
  models::{self, CreatableVersion, VersionDependency},
};
use octocrab::Octocrab;
use reqwest::{multipart::Part, Body};
use std::{env, process};
use tokio_util::codec::{BytesCodec, FramedRead};
use util::{get_keys, VersionType};

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

#[tokio::main]
async fn main() {
  match run().await {
    Ok(_) => (),
    Err(error) => {
      error!("{}", error);
      process::exit(1);
    }
  }
}

async fn run() -> Result<(), error::AxolotlError> {
  env::set_var("RUST_LOG", "info");

  pretty_env_logger::init();
  Keys::initialize()?;

  let arguments = Arguments::parse();

  match arguments.command {
    Commands::Mod { command } => match command {
      ModCommands::Init => 'init: {
        if Configuration::exists()? {
          warn!(
            "Found pre-existing configuration file {}",
            constants::CONFIGURATION
          );

          if !Confirm::new("Overwrite?").prompt()? {
            break 'init;
          }
        }

        Configuration::write(Configuration::default())?;

        info!(
          "The configuration file has been written to {}",
          constants::CONFIGURATION
        );
      }
      ModCommands::Publish => {
        let configuration = Configuration::read()?;
        let (keys, _) = get_keys()?;

        let changelog = match configuration.changelog {
          Some(changelog) => match changelog {
            Changelog::File { file } => {
              let file = ToRead::new(file)?;
              Some(file.read_to_string()?)
            }
            Changelog::Editor => Some(Editor::new("Write the changelog").prompt()?),
          },
          None => None,
        };

        let version = Text::new("Version").prompt()?;
        let tag = format!("v{}", version);

        let version_type = Select::new(
          "Version type",
          vec![VersionType::Release, VersionType::Beta, VersionType::Alpha],
        )
        .prompt()?;

        let asset_name = configuration
          .artifact
          .pattern
          .replace(constants::VERSION_REPLACE, version.as_str());
        let artifact = ToRead::new(configuration.artifact.folder.join(asset_name.clone()))?;

        if let Some(GitHub {
          repo: (user, repo),
          draft,
        }) = configuration.github
        {
          if let Some(token) = keys.github {
            let octocrab = Octocrab::builder().personal_token(token).build()?;
            let repo = octocrab.repos(user, repo);
            let releases = repo.releases();

            info!("Creating release");

            let release = releases
              .create(&tag.clone())
              .name(&tag.clone())
              .body(&changelog.clone().unwrap_or_default())
              .draft(draft)
              .prerelease(version_type != VersionType::Release)
              .send()
              .await?;

            info!(
              "Created release{} at {}",
              if draft { " draft" } else { "" },
              release.html_url
            );

            info!("Uploading artifact");

            let asset = releases
              .upload_asset(release.id.0, &asset_name, artifact.read()?.into())
              .label(&asset_name)
              .send()
              .await?;

            info!("Uploaded artifact to {}", asset.browser_download_url);
          } else {
            error!("A GitHub token was not provided, skipping distributing to GitHub Releases");
          }
        }

        if let Some(Modrinth {
          id,
          featured,
          dependencies,
        }) = configuration.modrinth
        {
          if let Some(token) = keys.modrinth {
            let config = apis::configuration::Configuration::with_api_key(token)?;

            apis::projects_api::check_project_validity(&config, &id).await?;

            // TODO
            let static_asset_name: &'static str = Box::leak(asset_name.clone().into_boxed_str());

            info!("Uploading version");

            let version = modrinth::create_version(
              &config,
              CreatableVersion {
                name: tag,
                version_number: version,
                changelog: Some(changelog),
                dependencies: dependencies
                  .iter()
                  .map(|d| <ModrinthDependency as Into<VersionDependency>>::into(d.clone()))
                  .collect(),
                game_versions: configuration.artifact.game_versions,
                version_type: version_type.into(),
                loaders: configuration.artifact.loaders,
                featured,
                status: Some(models::creatable_version::Status::Listed),
                requested_status: None,
                project_id: id,
                file_parts: vec![asset_name.clone()],
                primary_file: Some(asset_name),
              },
              Part::stream(Body::wrap_stream(FramedRead::new(
                artifact.open().await?,
                BytesCodec::new(),
              )))
              .file_name(static_asset_name),
            )
            .await?;

            info!(
              "Uploaded version to https://modrinth.com/project/{}/version/{}",
              version.project_id, version.id
            )
          } else {
            error!("A Modrinth token was not provided, skipping distributing to Modrinth");
          }
        }
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
            info!("{:#?}", util::get_keys()?.0);
          }
        }
        KeyCommands::Encryption { command } => match command {
          Some(EncryptionCommands::Enable) => {
            if raw.encrypted {
              info!("Your keys are already encrypted - disable and re-enable encryption to change the passphrase")
            } else {
              Keys::write(raw.encrypted(util::read_key_confirmation(true)?)?)?;
              info!("Encryption has been enabled");
            }
          }
          Some(EncryptionCommands::Disable) => {
            if !raw.encrypted {
              info!("Your keys have not been encrypted")
            } else {
              Keys::write(raw.decrypted(util::read_key()?)?)?;
              info!("Encryption has been disabled");
            }
          }
          None => info!(
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

          info!("The keys have been updated");
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

          info!("The keys have been updated");
        }
      }
    }
  }

  Ok(())
}
