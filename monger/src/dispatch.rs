use anyhow::Result;
use monger_core::Monger;
use self_update::backends::github::Update;

use crate::{util::file_exists_in_path, Defaults, Options};

impl Options {
    pub(super) fn dispatch(self) -> Result<()> {
        let monger = Monger::new()?;

        match self {
            Self::Clear { id } => {
                if monger.clear_database_files(&id)? {
                    println!("Cleared database files of {}", id);
                }
            }
            Self::Delete { id } => monger.delete_mongodb_version(&id)?,
            Self::Defaults(Defaults::Clear) => {
                if monger.clear_default_args()? {
                    println!("Cleared default args");
                }
            }
            Self::Defaults(Defaults::Get) => match monger.get_default_args()? {
                Some(args) => println!("default arguments:\n    {}", args),
                None => println!("no default arguments exist"),
            },
            Self::Defaults(Defaults::Set { args }) => {
                let args = args.join(" ");
                let trimmed_args = args.trim();

                if !trimmed_args.is_empty() {
                    monger.set_default_args(trimmed_args)?;

                    println!("default arguments set to:\n    {}", trimmed_args);
                }
            }
            Self::Download { url, id, force } => {
                monger.download_mongodb_version_from_url(&url, &id, force)?;
            }
            Self::Get {
                version,
                force,
                os,
                id,
            } => monger.download_mongodb_version(&version, force, os.as_deref(), id.as_deref())?,
            Self::List => list(&monger)?,
            Self::Prune => monger.prune()?,
            Self::Run { id, bin, bin_args } => {
                return Err(monger
                    .exec_command(&bin, bin_args.into_iter().map(Into::into).collect(), &id)
                    .into());
            }
            Self::SelfUpdate => {
                let status = Update::configure()
                    .repo_owner("saghm")
                    .repo_name("monger")
                    .current_version(env!("CARGO_PKG_VERSION"))
                    .bin_name(env!("CARGO_PKG_NAME"))
                    .show_download_progress(true)
                    .build()?
                    .update()?;

                if status.uptodate() {
                    println!("Already have the latest version");
                } else {
                    println!("Downloaded and installed {}", status.version());
                }
            }
            Self::Start { id, mongod_args } => {
                monger.start_mongod(
                    mongod_args.into_iter().map(Into::into).collect(),
                    &id,
                    true,
                )?;
            }
        }

        Ok(())
    }
}

fn list(monger: &Monger) -> Result<()> {
    let mut versions: Vec<_> = monger
        .list_versions()?
        .into_iter()
        .map(|s| s.to_string_lossy().into_owned())
        .collect();
    versions.sort();

    if file_exists_in_path("mongod") {
        versions.push("system".to_string());
    }

    print!("installed versions:");

    if versions.is_empty() {
        println!(" none");
    } else {
        println!();

        for version in versions {
            println!("    {}", version);
        }
    }
    Ok(())
}
