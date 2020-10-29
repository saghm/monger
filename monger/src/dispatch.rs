use anyhow::Result;
use monger_core::{LogFile, LogFileType, Monger};
use rand::seq::IteratorRandom;
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
            Self::Start {
                id,
                save_log,
                mongod_args,
            } => {
                let port = mongod_args
                    .iter()
                    .position(|arg| arg == "--port")
                    .and_then(|i| mongod_args.get(i + 1))
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(27017);

                let mut mongod_args: Vec<_> = mongod_args.into_iter().map(Into::into).collect();

                let save_log = if save_log {
                    mongod_args.push("--fork".into());

                    let cluster_id: String = (0..8)
                        .map(|_| alpha_numeric().choose(&mut rand::thread_rng()).unwrap())
                        .collect();

                    monger.clear_cluster_logs(&cluster_id)?;

                    println!("NOTE: log file saved under cluster id '{}'\n", cluster_id);

                    Some(LogFile {
                        cluster_id,
                        port,
                        node_type: LogFileType::DataNode,
                    })
                } else {
                    None
                };

                monger.start_mongod(mongod_args, &id, true, save_log)?;
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

fn alpha_numeric() -> impl Iterator<Item = char> {
    ('0'..'9').chain('A'..'Z').chain('a'..'z')
}
