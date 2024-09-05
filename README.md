# Axolotol

A CLI tool for mod distribution

## Table of Contents

- [Axolotol](#axolotol)
  - [Table of Contents](#table-of-contents)
  - [Getting Started](#getting-started)
  - [Setup](#setup)
  - [Using](#using)
    - [Publishing](#publishing)
  - [Encryption](#encryption)
    - [Enabling](#enabling)
    - [Disabling](#disabling)

## Getting Started

To install Axolotl, first install `cargo` and run `cargo install axolotl-cli`. The `axolotl` command should now be available.

## Setup

> Axolotl currently supports Modrinth and GitHub Releases; Curseforge support is not planned.

Create personal access tokens (PATs) for whichever of the following you plan to distribute to:

Scopes required for [Modrinth](https://modrinth.com/settings/pats):

- Read projects
- Create versions

Scopes required for [GitHub](https://github.com/settings/tokens?type=beta):

- Repositories
  - Contents: read and write

If you chose to use Modrinth, run `axolotl key set modrinth` (or `axolotl k s m`) and enter your Modrinth PAT. If you chose to use Modrinth, run `axolotl key set github` (or `axolotl k s g`) and enter your GitHub PAT.

To remove a key. run `axolotl key remove <modrinth/github>` (or `axolotl k r <m/g>)`. It will be permanently removed.

## Using

At the root of the project you want to distribute, run `axolotl mod init` (or `axolotl m i`). This should create an `axolotl.toml` file. This is the format and default content:

```toml
[artifact] # required
folder = "build/libs" # required; the folder with the build artifact
pattern = "mod-#.jar" # required; the artifact file name (# will be replaced with the version)
game_versions = ["1.xx"] # required; the supported minecraft versions
loaders = ["fabric", "quilt", "forge", "neoforge"] # required; the supported modloaders

[changelog] # optional
type = "editor" # required; "editor" (prompt to open an editor when publishing) or "file" (use the contents of a file)
# file = "path/to/file.md" - required if type is "file"

[modrinth] # optional
id = "modrinth project id" # required; the modrinth project id of the target project (can be found by clicking the three dots on the project page)
featured = true # optional (default true); whether the new version should be featured

[github] # optional
repo = ["user", "repo"] # required; the target repository
draft = true # optional (default true); whether the release should be marked as a draft to review before publishing
```

### Publishing

To publish your mod, run `axolotl mod publish` (or `axolotl m p`).

## Encryption

Keys are stored in plaintext by default, but they may be encrypted with a passphrase (using [age](https://crates.io/crates/age)). Encryption makes most operations slower to start up, as keys need to be decrypted, but it provides **much** greater security.

### Enabling

Run `axolotl keys encryption enable` (or `axolotl k e e`). You will be prompted to create a passphrase - this passphrase will be requested when any action requiring the decryption of the keys is done.

### Disabling

Run `axolotl keys encryption disable` (or `axolotl k e d`). You will be prompted for your passphrase.
