use std::ffi::OsStr;

use semver::Version;

use client::HttpClient;
use error::Result;
use fs::Fs;
use os::OperatingSystem;

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
        if self.fs.version_exists(version_str) {
            return Ok(());
        }

        let version = Version::parse(version_str)?;

        let url = OperatingSystem::get()?.download_url(&version);
        let file = url.filename();
        let url: String = url.into();
        let data = self.client.download_file(&url)?;
        self.fs.write_mongodb_download(&file, &data[..], version_str)?;

        Ok(())
    }

    pub fn delete_mongodb_version(&self, version: &str) -> Result<()> {
        self.fs.delete_mongodb_version(version)
    }

    pub fn exec<I, S>(&self, binary_name: &str, args: I, version: &str) -> Result<()>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        self.fs.exec(binary_name, args, version)
    }
}
