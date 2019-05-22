use std::{
    ffi::{OsStr, OsString},
    io::ErrorKind::NotFound,
};

use regex::Regex;
use semver::Version;
use soup::{NodeExt, QueryBuilderExt, Soup};

use crate::{
    client::HttpClient,
    error::{Error, ErrorKind, Result},
    fs::Fs,
    os::OperatingSystem,
    process::exec_command,
    util::{parse_major_minor_version, select_newer_version},
};

const MONGODB_VERSION_LIST_URL: &str = "https://dl.mongodb.org/dl/src";

lazy_static! {
    static ref MONGODB_SEMVER_REGEX: Regex =
        Regex::new(r"src/mongodb-src-r(\d+\.\d+\.\d+)\.tar\.gz$").unwrap();
}

pub struct Monger {
    client: HttpClient,
    fs: Fs,
}

impl Monger {
    pub fn new() -> Result<Self> {
        Ok(Self {
            client: HttpClient::new()?,
            fs: Fs::new().build()?,
        })
    }

    pub fn download_mongodb_version(&self, version_str: &str) -> Result<()> {
        let version = if version_str == "latest" {
            self.find_latest_mongodb_version()?
        } else if let Some((major, minor)) = parse_major_minor_version(version_str) {
            self.find_latest_matching_version(major, minor)?
        } else {
            Version::parse(version_str).map_err(|_| {
                let err: Error = ErrorKind::VersionNotFound(version_str.to_string()).into();
                err
            })?
        };

        if self.fs.version_exists(&version.to_string()) {
            return Ok(());
        }

        self.download_and_write(OperatingSystem::get(&version)?, version)
    }

    pub fn download_and_write(&self, os: OperatingSystem, version: Version) -> Result<()> {
        let version_str = version.to_string();

        let url = os.download_url(&version);
        let file = url.filename();
        let dir = url.dirname();
        let url: String = url.into();
        let data = self.client.download_file(&url, &version_str)?;

        self.fs
            .write_mongodb_download(&file, &dir, &data[..], &version_str)?;

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

        bail!(ErrorKind::VersionNotFound(format!("{}.{}", major, minor)))
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
            bail!(ErrorKind::InvalidHtml(MONGODB_VERSION_LIST_URL.to_string()))
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

    fn process_args<I>(&self, mut args: I, version: &str) -> Result<Vec<OsString>>
    where
        I: Iterator<Item = OsString>,
    {
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

        return Ok(processed_args);
    }

    pub fn start_mongod<I>(&self, args: I, version: &str) -> Result<()>
    where
        I: Iterator<Item = OsString>,
    {
        let processed_args = self.process_args(args, version)?;

        if version == "system" {
            self.system_exec("mongod", processed_args)
        } else {
            self.exec("mongod", processed_args, &version)
        }
    }

    fn system_exec<I, S>(&self, binary_name: &str, args: I) -> Result<()>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        exec_command(binary_name, args.into_iter().collect())
    }

    pub fn exec<I, S>(&self, binary_name: &str, args: I, version: &str) -> Result<()>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        if version == "system" {
            return self.system_exec(binary_name, args);
        }

        match self
            .fs
            .exec(binary_name, args.into_iter().collect(), &version)
        {
            Err(Error(ErrorKind::Io(ref io_err), _)) if io_err.kind() == NotFound => bail!(
                ErrorKind::BinaryNotFound(binary_name.to_string(), version.to_string(),)
            ),
            other => other,
        }
    }
}
