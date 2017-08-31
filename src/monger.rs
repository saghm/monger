use std::ffi::{OsStr, OsString};
use std::io::ErrorKind::NotFound;

use semver::Version;
use serde_json::Value;

use client::HttpClient;
use error::{Error, ErrorKind, Result};
use fs::Fs;
use os::OperatingSystem;
use process::exec_command;

const MONGODB_GIT_TAGS_URL: &str = "https://api.github.com/repos/mongodb/mongo/tags";

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
        } else {
            Version::parse(version_str)?
        };

        let version_str = format!("{}", version);

        if self.fs.version_exists(&version_str) {
            return Ok(());
        }

        let url = OperatingSystem::get()?.download_url(&version);
        let file = url.filename();
        let dir = url.dirname();
        let url: String = url.into();
        let data = self.client.download_file(&url)?;

        self.fs.write_mongodb_download(
            &file,
            &dir,
            &data[..],
            &version_str,
        )?;

        Ok(())
    }

    fn find_latest_mongodb_version(&self) -> Result<Version> {
        let json = self.client.get_json(MONGODB_GIT_TAGS_URL)?;
        let array = match json {
            Value::Array(values) => values,
            _ => bail!(ErrorKind::InvalidJson(MONGODB_GIT_TAGS_URL.to_string())),
        };

        for value in array {
            let name = match value.get("name") {
                Some(&Value::String(ref s)) => s,
                _ => continue,
            };

            if !name.starts_with('r') {
                continue;
            }

            let version = match Version::parse(&name[1..]) {
                Ok(v) => v,
                Err(_) => continue,
            };

            // Only even-numbered minor versions without an rc tag are stable.
            if version.minor % 2 == 0 && version.pre.is_empty() && version.build.is_empty() {
                return Ok(version);
            }
        }

        bail!(ErrorKind::InvalidJson(MONGODB_GIT_TAGS_URL.to_string()))
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

    pub fn start_mongod<I>(&self, args: I, version: &str) -> Result<()>
    where
        I: Iterator<Item = OsString>,
    {
        if version == "system" {
            let db_dir = self.fs.create_or_get_db_dir("system")?;

            return self.system_exec(
                "mongod",
                args.chain(vec!["--dbpath".into(), db_dir.into_os_string()]),
            );
        }

        let db_dir = self.fs.create_or_get_db_dir(version)?;
        self.exec(
            "mongod",
            args.chain(vec!["--dbpath".into(), db_dir.into_os_string()]),
            version,
        )
    }

    fn system_exec<I, S>(&self, binary_name: &str, args: I) -> Result<()>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        exec_command::<_, _, &str>(binary_name, args, None)
    }

    pub fn exec<I, S>(&self, binary_name: &str, args: I, version: &str) -> Result<()>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        if version == "system" {
            return self.system_exec(binary_name, args);
        }

        match self.fs.exec(binary_name, args, version) {
            Err(Error(ErrorKind::Io(ref io_err), _)) if io_err.kind() == NotFound => {
                bail!(ErrorKind::BinaryNotFound(
                    binary_name.to_string(),
                    version.to_string(),
                ))
            }
            other => other,
        }
    }
}
