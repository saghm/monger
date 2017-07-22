mod arch;
mod macos;
mod linux;
mod windows;

use semver::Version;

use self::macos::MacOsType;
use self::linux::LinuxType;
use self::windows::WindowsType;
use super::url::UrlBuilder;
use super::util::FileExtension;

#[derive(Debug)]
enum OperatingSystem {
    Linux(LinuxType),
    MacOs(MacOsType),
    Windows(WindowsType),
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

    pub fn download_url(&self, version: &Version) -> String {
        let mut builder = UrlBuilder::new(self.name(), self.extension().name());

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

    fn matches_url(url: &str, os: OperatingSystem) {
        let version = Version::parse("3.4.6").unwrap();

        assert_eq!(url, os.download_url(&version));
    }

    //
    // Linux URLs
    //

    #[test]
    fn amazon_linux_url() {
        matches_url(
            "https://fastdl.mongodb.org/linux/mongodb-linux-x86_64-amazon-3.4.6.tgz",
            OperatingSystem::Linux(LinuxType::Amazon),
        );
    }

    #[test]
    fn debian7_linux_url() {
        matches_url(
            "https://fastdl.mongodb.org/linux/mongodb-linux-x86_64-debian71-3.4.6.tgz",
            OperatingSystem::Linux(LinuxType::Debian7),
        );
    }

    #[test]
    fn debian8_linux_url() {
        matches_url(
            "https://fastdl.mongodb.org/linux/mongodb-linux-x86_64-debian81-3.4.6.tgz",
            OperatingSystem::Linux(LinuxType::Debian8),
        );
    }

    #[test]
    fn legacy_linux_url() {
        matches_url(
            "https://fastdl.mongodb.org/linux/mongodb-linux-x86_64-3.4.6.tgz",
            OperatingSystem::Linux(LinuxType::Legacy),
        );
    }

    #[test]
    fn rhel6_linux_url() {
        matches_url(
            "https://fastdl.mongodb.org/linux/mongodb-linux-x86_64-rhel62-3.4.6.tgz",
            OperatingSystem::Linux(LinuxType::Rhel6),
        );
    }
    #[test]
    fn rhel7_linux_url() {
        matches_url(
            "https://fastdl.mongodb.org/linux/mongodb-linux-x86_64-rhel70-3.4.6.tgz",
            OperatingSystem::Linux(LinuxType::Rhel7),
        );
    }

    #[test]
    fn suse11_linux_url() {
        matches_url(
            "https://fastdl.mongodb.org/linux/mongodb-linux-x86_64-suse11-3.4.6.tgz",
            OperatingSystem::Linux(LinuxType::Suse11),
        );
    }

    #[test]
    fn suse12_linux_url() {
        matches_url(
            "https://fastdl.mongodb.org/linux/mongodb-linux-x86_64-suse12-3.4.6.tgz",
            OperatingSystem::Linux(LinuxType::Suse12),
        );
    }

    #[test]
    fn ubuntu1204_linux_url() {
        matches_url(
            "https://fastdl.mongodb.org/linux/mongodb-linux-x86_64-ubuntu1204-3.4.6.tgz",
            OperatingSystem::Linux(LinuxType::Ubuntu1204),
        );
    }

    #[test]
    fn ubunutu1404_linux_url() {
        matches_url(
            "https://fastdl.mongodb.org/linux/mongodb-linux-x86_64-ubuntu1404-3.4.6.tgz",
            OperatingSystem::Linux(LinuxType::Ubuntu1404),
        );
    }

    #[test]
    fn ubunutu1604_arm_linux_url() {
        matches_url(
            "https://fastdl.mongodb.org/linux/mongodb-linux-arm64-ubuntu1604-3.4.6.tgz",
            OperatingSystem::Linux(LinuxType::Ubuntu1604(Architecture::Arm)),
        );
    }

    #[test]
    fn ubuntu1604_x86_64_linux_url() {
        matches_url(
            "https://fastdl.mongodb.org/linux/mongodb-linux-x86_64-ubuntu1604-3.4.6.tgz",
            OperatingSystem::Linux(LinuxType::Ubuntu1604(Architecture::X86_64)),
        );
    }

    //
    // MacOS URLs
    //

    #[test]
    fn nonssl_macos_url() {
        matches_url(
            "https://fastdl.mongodb.org/osx/mongodb-osx-x86_64-3.4.6.tgz",
            OperatingSystem::MacOs(MacOsType::NonSsl),
        );
    }

    #[test]
    fn ssl_macos_url() {
        matches_url(
            "https://fastdl.mongodb.org/osx/mongodb-osx-ssl-x86_64-3.4.6.tgz",
            OperatingSystem::MacOs(MacOsType::Ssl),
        );
    }

    //
    // Windows URLs
    //

    #[test]
    fn server2008_windows_url() {
        matches_url(
            "https://fastdl.mongodb.org/win32/mongodb-win32-x86_64-3.4.6-signed.msi",
            OperatingSystem::Windows(WindowsType::Server2008),
        );
    }


    #[test]
    fn server2008_r2_windows_url() {
        matches_url(
            "https://fastdl.mongodb.org/win32/mongodb-win32-x86_64-2008plus-3.4.6-signed.msi",
            OperatingSystem::Windows(WindowsType::Server2008R2),
        );
    }

    #[test]
    fn server2008_r2_ssl_windows_url() {
        matches_url(
            "https://fastdl.mongodb.org/win32/mongodb-win32-x86_64-2008plus-ssl-3.4.6-signed.msi",
            OperatingSystem::Windows(WindowsType::Server2008R2Ssl),
        );
    }
}
