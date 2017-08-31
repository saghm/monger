use std::ffi::{OsStr, OsString};
use std::io::ErrorKind::NotFound;

use semver::Version;
use serde_json::Value;

use client::HttpClient;
use error::{Error, ErrorKind, Result};
use fs::Fs;
use os::OperatingSystem;
use process::exec_command;
use tags::Tags;
use util::select_newer_version;

const MONGODB_GIT_TAGS_URL: &str = "https://api.github.com/repos/mongodb/mongo/tags?per_page=100";

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
        let mut page = Some(MONGODB_GIT_TAGS_URL.to_string());
        let mut newest_stable = None;
        let mut newest_dev = None;

        while let Some(current_page) = page {
            let response = self.client.get(&current_page)?;
            let tags = Tags::from_response(response)?;

            {
                let array = match tags.get_value() {
                    &Value::Array(ref values) => values,
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

                    // Skip release candidates.
                    if !version.pre.is_empty() || !version.build.is_empty() {
                        continue;
                    }

                    if version.minor % 2 == 0 {
                        newest_stable = Some(select_newer_version(newest_stable, version));
                    } else if version.minor % 2 == 1 && newest_dev.is_none() {
                        newest_dev = Some(version);
                    }

                    newest_stable = match newest_stable {
                        Some(stable_version) => {
                            if let Some(ref dev_version) = newest_dev {
                                // Since there will only be one dev version in development at a
                                // given time, the newest stable version will never be older than
                                // one minor version less than the most recent dev version.
                                if dev_version.major == stable_version.major &&
                                    dev_version.minor <= stable_version.minor + 1
                                {
                                    return Ok(stable_version);
                                }
                            }

                            Some(stable_version)
                        }
                        None => None,
                    };
                }
            }

            page = tags.next_page_url();
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
