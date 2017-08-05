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

    pub fn download_mongodb_version(&self, version: &str) -> Result<()> {
        let version = Version::parse(version)?;
        let url = OperatingSystem::get()?.download_url(&version);
        let file = url.filename();
        let url: String = url.into();
        let data = self.client.download_file(&url)?;
        self.fs.write_mongodb_download(&file, &data[..], &version)?;

        Ok(())
    }
}