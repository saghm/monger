#[macro_use]
mod util;

mod client;
pub mod error;
mod fs;
pub mod os;
pub mod process;
mod url;

use std::{ffi::OsString, io::ErrorKind::NotFound, process::Child};

use lazy_static::lazy_static;
use regex::Regex;
use semver::Version;
use soup::{NodeExt, QueryBuilderExt, Soup};

use crate::{
    client::HttpClient,
    error::{Error, Result},
    fs::Fs,
    os::OperatingSystem,
    process::{exec_command, run_background_command},
    util::{parse_major_minor_version, select_newer_version},
};

const MONGODB_VERSION_LIST_URL: &str = "https://dl.mongodb.org/dl/src";

lazy_static! {
    static ref MONGODB_SEMVER_REGEX: Regex =
        Regex::new(r"src/mongodb-src-r(\d+\.\d+\.\d+)\.tar\.gz$").unwrap();
}

#[derive(Debug)]
pub struct Monger {
    client: HttpClient,
    fs: Fs,
}

impl Monger {
    pub fn new() -> Result<Self> {
        Ok(Self {
            client: HttpClient::new()?,
            fs: Fs::builder().build()?,
        })
    }

    pub fn clear_database_files(&self, version_str: &str) -> Result<bool> {
        self.fs.clear_db_dir(version_str)
    }

    pub fn download_mongodb_version(
        &self,
        version_str: &str,
        force: bool,
        os: Option<&str>,
        id: Option<&str>,
    ) -> Result<()> {
        let version = if version_str == "latest" {
            self.find_latest_mongodb_version()?
        } else if let Some((major, minor)) = parse_major_minor_version(version_str) {
            self.find_latest_matching_version(major, minor)?
        } else {
            crate::util::parse_version(version_str)?
        };

        let id = id
            .map(ToString::to_string)
            .unwrap_or_else(|| version.to_string());

        if self.fs.version_exists(&id) {
            if force {
                self.delete_mongodb_version(&id)?;
            } else {
                return Ok(());
            }
        }

        let os = if let Some(os_name) = os {
            OperatingSystem::from_name(os_name).unwrap()
        } else {
            OperatingSystem::get(&version)?
        };

        let url = os.download_url(&version);
        let file = url.filename();
        let dir = url.dirname();
        let url: String = url.into();
        let data = self.client.download_file(&url, &version_str)?;

        self.fs
            .write_mongodb_download(&file, &dir, &data[..], &id)?;

        Ok(())
    }

    fn find_latest_matching_version(&self, major: u64, minor: u64) -> Result<Version> {
        let response = self.client.get(MONGODB_VERSION_LIST_URL)?;
        let soup = Soup::from_reader(response)?;

        let matches = soup
            .tag("a")
            .attr("href", MONGODB_SEMVER_REGEX.clone())
            .find_all()
            .map(|item| {
                // We know the capture we're looking for will exist (and will be a valid semver
                // string) due to Soup finding it as a match, so it's safe to unwrap
                // here.
                Version::parse(
                    &*MONGODB_SEMVER_REGEX
                        .captures(&item.text())
                        .unwrap()
                        .get(1)
                        .unwrap()
                        .as_str(),
                )
                .unwrap()
            });

        for version in matches {
            if major == version.major && minor == version.minor {
                return Ok(version);
            }
        }

        Err(Error::VersionNotFound {
            version: format!("{}.{}", major, minor),
        })
    }

    fn find_latest_mongodb_version(&self) -> Result<Version> {
        let response = self.client.get(MONGODB_VERSION_LIST_URL)?;
        let soup = Soup::from_reader(response)?;

        let mut newest_stable = None;
        let mut newest_dev = None;

        for version in soup
            .tag("a")
            .attr("href", MONGODB_SEMVER_REGEX.clone())
            .find_all()
            .map(|item| {
                // We know the capture we're looking for will exist (and will be a valid semver
                // string) due to Soup finding it as a match, so it's safe to unwrap
                // here.
                Version::parse(
                    &*MONGODB_SEMVER_REGEX
                        .captures(&item.text())
                        .unwrap()
                        .get(1)
                        .unwrap()
                        .as_str()
                        .to_string(),
                )
                .unwrap()
            })
        {
            if version.minor % 2 == 0 {
                newest_stable = Some(select_newer_version(newest_stable, version));
            } else {
                newest_dev = Some(version);
            }

            // Since there will only be one dev version in development at a given time, the newest
            // stable version will never be older than one minor version less than the most recent
            // dev version.
            if let Some(ref stable_version) = newest_stable {
                if let Some(ref dev_version) = newest_dev {
                    if dev_version.major == stable_version.major
                        && dev_version.minor == stable_version.minor + 1
                    {
                        return Ok(newest_stable.unwrap());
                    }
                }
            }
        }

        if let Some(version) = newest_stable {
            Ok(version)
        } else {
            Err(Error::InvalidHtml {
                url: MONGODB_VERSION_LIST_URL.to_string(),
            })
        }
    }

    pub fn delete_mongodb_version(&self, version: &str) -> Result<()> {
        if self.fs.delete_mongodb_version(version)? {
            println!("Deleted version {}", version);
        }

        Ok(())
    }

    pub fn list_versions(&self) -> Result<Vec<OsString>> {
        self.fs.list_versions()
    }

    pub fn prune(&self) -> Result<()> {
        self.fs.prune()
    }

    fn process_args(&self, args: Vec<OsString>, version: &str) -> Result<Vec<OsString>> {
        let mut args = args.into_iter();

        let mut processed_args = Vec::new();
        let mut found_dbpath = false;

        while let Some(arg) = args.next() {
            if arg.as_os_str() == "--dbpath" {
                processed_args.push(arg);
                found_dbpath = true;
                break;
            }

            processed_args.push(arg);
        }

        if found_dbpath {
            processed_args.extend(args);
        } else {
            processed_args.push("--dbpath".into());
            processed_args.push(self.fs.create_or_get_db_dir(version)?.into_os_string());
        }

        Ok(processed_args)
    }

    pub fn start_mongod(&self, args: Vec<OsString>, version: &str, exec: bool) -> Result<Child> {
        let processed_args = self.process_args(args, version)?;

        if exec {
            Err(self.exec_command("mongod", processed_args, version))
        } else {
            self.run_background_command("mongod", processed_args, version)
        }
    }

    pub fn run_background_command(
        &self,
        binary_name: &str,
        args: Vec<OsString>,
        version: &str,
    ) -> Result<Child> {
        if version == "system" {
            return run_background_command(binary_name, args, std::env::current_dir()?);
        }

        let result =
            self.fs
                .run_background_command(binary_name, args.into_iter().collect(), version);

        match result {
            Err(Error::Io { ref inner }) if inner.kind() == NotFound => {
                Err(Error::BinaryNotFound {
                    binary: binary_name.into(),
                    version: version.into(),
                })
            }
            other => other,
        }
    }

    pub fn exec_command(&self, binary_name: &str, args: Vec<OsString>, version: &str) -> Error {
        let dir = match std::env::current_dir() {
            Ok(dir) => dir,
            Err(e) => return e.into(),
        };

        let error = if version == "system" {
            exec_command(binary_name, args, dir)
        } else {
            self.fs
                .exec_command(binary_name, args.into_iter().collect(), version)
        };

        match error {
            Error::Io { ref inner } if inner.kind() == NotFound => Error::BinaryNotFound {
                binary: binary_name.into(),
                version: version.into(),
            },
            other => other,
        }
    }
}
