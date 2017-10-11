mod arch;
#[allow(dead_code)]
mod macos;
mod linux;
#[allow(dead_code)]
mod windows;

use std::env::consts;

use semver::Version;

use error::{ErrorKind, Result};
pub use self::macos::MacOsType;
pub use self::linux::LinuxType;
pub use self::windows::WindowsType;
use super::url::{Url, UrlBuilder};
use util::FileExtension;

#[derive(Debug)]
pub enum OperatingSystem {
    Linux(LinuxType),

    #[allow(dead_code)]
    MacOs(MacOsType),

    #[allow(dead_code)]
    Windows(WindowsType),
}

impl OperatingSystem {
    pub fn get(version: &Version) -> Result<Self> {
        match consts::OS {
            "linux" => LinuxType::get().map(OperatingSystem::Linux),
            "macos" => {
                // MongoDB releases before 3.0 for MacOS did not link to an SSL library.
                //
                // TODO: Use pkg-config to check if SSL is installed as well.
                let macos_type = if version.major < 3 {
                    MacOsType::NonSsl
                } else {
                    MacOsType::Ssl
                };

                Ok(OperatingSystem::MacOs(macos_type))
            },
            "windows" => bail!(ErrorKind::UnsupportedOs("windows".to_string())),
            s => bail!(ErrorKind::UnsupportedOs(s.to_string())),
        }
    }
}

impl OperatingSystem {
    fn extension(&self) -> FileExtension {
        match *self {
            OperatingSystem::Linux(_) |
            OperatingSystem::MacOs(_) => FileExtension::Tgz,
            OperatingSystem::Windows(_) => FileExtension::Msi,
        }
    }

    fn name(&self) -> &'static str {
        match *self {
            OperatingSystem::Linux(_) => "linux",
            OperatingSystem::MacOs(_) => "osx",
            OperatingSystem::Windows(_) => "win32",
        }
    }

    pub fn download_url(&self, version: &Version) -> Url {
        let mut builder = UrlBuilder::new(self.name(), self.extension().name(), version);

        builder.add_distro_path_item("mongodb".to_string());
        builder.add_distro_path_item(self.name().to_string());

        let url_path = match *self {
            OperatingSystem::Linux(ref os_type) => os_type.url_path(version),
            OperatingSystem::MacOs(ref os_type) => os_type.url_path(version),
            OperatingSystem::Windows(ref os_type) => os_type.url_path(version),
        };

        for item in url_path {
            builder.add_distro_path_item(item);
        }

        builder.build()
    }
}

#[cfg(test)]
mod tests {
    use semver::Version;

    use super::arch::Architecture;
    use super::OperatingSystem;
    use super::linux::LinuxType;
    use super::macos::MacOsType;
    use super::windows::WindowsType;

    fn matches_url(url: &str, os: OperatingSystem, version: Version) -> String {
        let download_url = os.download_url(&version);
        let dirname = download_url.dirname();

        assert_eq!(url, String::from(download_url));
        dirname
    }

    //
    // Linux URLs
    //

    #[test]
    fn amazon_linux_url() {
        matches_url(
            "https://fastdl.mongodb.org/linux/mongodb-linux-x86_64-amazon-3.4.6.tgz",
            OperatingSystem::Linux(LinuxType::Amazon),
            version!(3, 4, 6),
        );
    }

    #[test]
    fn debian7_linux_url() {
        matches_url(
            "https://fastdl.mongodb.org/linux/mongodb-linux-x86_64-debian71-3.4.6.tgz",
            OperatingSystem::Linux(LinuxType::Debian7),
            version!(3, 4, 6),
        );
    }

    #[test]
    fn debian8_linux_url() {
        matches_url(
            "https://fastdl.mongodb.org/linux/mongodb-linux-x86_64-debian81-3.4.6.tgz",
            OperatingSystem::Linux(LinuxType::Debian8),
            version!(3, 4, 6),
        );
    }

    #[test]
    fn legacy_linux_url() {
        matches_url(
            "https://fastdl.mongodb.org/linux/mongodb-linux-x86_64-3.4.6.tgz",
            OperatingSystem::Linux(LinuxType::Legacy),
            version!(3, 4, 6),
        );
    }

    #[test]
    fn rhel6_linux_url() {
        matches_url(
            "https://fastdl.mongodb.org/linux/mongodb-linux-x86_64-rhel62-3.4.6.tgz",
            OperatingSystem::Linux(LinuxType::Rhel6),
            version!(3, 4, 6),
        );
    }

    #[test]
    fn rhel7_linux_url() {
        matches_url(
            "https://fastdl.mongodb.org/linux/mongodb-linux-x86_64-rhel70-3.4.6.tgz",
            OperatingSystem::Linux(LinuxType::Rhel7),
            version!(3, 4, 6),
        );
    }

    #[test]
    fn suse11_linux_url() {
        matches_url(
            "https://fastdl.mongodb.org/linux/mongodb-linux-x86_64-suse11-3.4.6.tgz",
            OperatingSystem::Linux(LinuxType::Suse11),
            version!(3, 4, 6),
        );
    }

    #[test]
    fn suse12_linux_url() {
        matches_url(
            "https://fastdl.mongodb.org/linux/mongodb-linux-x86_64-suse12-3.4.6.tgz",
            OperatingSystem::Linux(LinuxType::Suse12),
            version!(3, 4, 6),
        );
    }

    #[test]
    fn ubuntu1204_linux_url() {
        matches_url(
            "https://fastdl.mongodb.org/linux/mongodb-linux-x86_64-ubuntu1204-3.4.6.tgz",
            OperatingSystem::Linux(LinuxType::Ubuntu1204),
            version!(3, 4, 6),
        );
    }

    #[test]
    fn ubunutu1404_linux_url() {
        matches_url(
            "https://fastdl.mongodb.org/linux/mongodb-linux-x86_64-ubuntu1404-3.4.6.tgz",
            OperatingSystem::Linux(LinuxType::Ubuntu1404),
            version!(3, 4, 6),
        );
    }

    #[test]
    fn ubunutu1604_arm_linux_url() {
        matches_url(
            "https://fastdl.mongodb.org/linux/mongodb-linux-arm64-ubuntu1604-3.4.6.tgz",
            OperatingSystem::Linux(LinuxType::Ubuntu1604(Architecture::Arm)),
            version!(3, 4, 6),
        );
    }

    #[test]
    fn ubuntu1604_x86_64_linux_url() {
        matches_url(
            "https://fastdl.mongodb.org/linux/mongodb-linux-x86_64-ubuntu1604-3.4.6.tgz",
            OperatingSystem::Linux(LinuxType::Ubuntu1604(Architecture::X86_64)),
            version!(3, 4, 6),
        );
    }

    //
    // MacOS URLs
    //

    #[test]
    fn nonssl_osx_url() {
        let dirname = matches_url(
            "https://fastdl.mongodb.org/osx/mongodb-osx-x86_64-3.4.6.tgz",
            OperatingSystem::MacOs(MacOsType::NonSsl),
            version!(3, 4, 6),
        );

        assert_eq!(dirname, "mongodb-osx-x86_64-3.4.6");
    }

    #[test]
    fn ssl_osx_url() {
        let dirname = matches_url(
            "https://fastdl.mongodb.org/osx/mongodb-osx-ssl-x86_64-3.4.6.tgz",
            OperatingSystem::MacOs(MacOsType::Ssl),
            version!(3, 4, 6),
        );

        assert_eq!(dirname, "mongodb-osx-x86_64-3.4.6");
    }

    #[test]
    fn ssl_macos_url() {
        let dirname = matches_url(
            "https://fastdl.mongodb.org/osx/mongodb-osx-ssl-x86_64-3.5.4.tgz",
            OperatingSystem::MacOs(MacOsType::Ssl),
            version!(3, 5, 4),
        );

        assert_eq!(dirname, "mongodb-macOS-x86_64-3.5.4");
    }

    //
    // Windows URLs
    //

    #[test]
    fn server2008_windows_url() {
        matches_url(
            "https://fastdl.mongodb.org/win32/mongodb-win32-x86_64-3.4.6-signed.msi",
            OperatingSystem::Windows(WindowsType::Server2008),
            version!(3, 4, 6),
        );
    }

    #[test]
    fn server2008_r2_windows_url() {
        matches_url(
            "https://fastdl.mongodb.org/win32/mongodb-win32-x86_64-2008plus-3.4.6-signed.msi",
            OperatingSystem::Windows(WindowsType::Server2008R2),
            version!(3, 4, 6),
        );
    }

    #[test]
    fn server2008_r2_ssl_windows_url() {
        matches_url(
            "https://fastdl.mongodb.org/win32/mongodb-win32-x86_64-2008plus-ssl-3.4.6-signed.msi",
            OperatingSystem::Windows(WindowsType::Server2008R2Ssl),
            version!(3, 4, 6),
        );
    }
}
