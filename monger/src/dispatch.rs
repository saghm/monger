use monger_core::{error::Result, Monger};

use crate::{util::file_exists_in_path, Options};

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
                return Err(monger.exec_command(
                    &bin,
                    bin_args.into_iter().map(Into::into).collect(),
                    &id,
                ));
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
