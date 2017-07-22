use semver::Version;

use super::arch::Architecture;

#[derive(Debug)]
pub enum LinuxType {
    Amazon,
    Debian7,
    Debian8,
    Legacy,
    Rhel6,
    Rhel7,
    Suse11,
    Suse12,
    Ubuntu1204,
    Ubuntu1404,
    Ubuntu1604(Architecture),
}

impl LinuxType {
    fn architecture(&self) -> Architecture {
        if let LinuxType::Ubuntu1604(arch) = *self {
            return arch;
        } else {
            return Architecture::X86_64;
        }
    }

    fn name(&self) -> Option<&'static str> {
        match *self {
            LinuxType::Amazon => Some("amazon"),
            LinuxType::Debian7 => Some("debian71"),
            LinuxType::Debian8 => Some("debian81"),
            LinuxType::Legacy => None,
            LinuxType::Rhel6 => Some("rhel62"),
            LinuxType::Rhel7 => Some("rhel70"),
            LinuxType::Suse11 => Some("suse11"),
            LinuxType::Suse12 => Some("suse12"),
            LinuxType::Ubuntu1204 => Some("ubuntu1204"),
            LinuxType::Ubuntu1404 => Some("ubuntu1404"),
            LinuxType::Ubuntu1604(_) => Some("ubuntu1604"),
        }
    }

    pub fn url_path(&self, version: &Version) -> Vec<String> {
        let mut path = vec![self.architecture().name().to_string()];

        if let Some(name) = self.name() {
            path.push(name.to_string());
        }

        path.push(format!("{}", version));
        path
    }
}

#[cfg(test)]
mod tests {
    use semver::Version;

    use os::arch::Architecture;
    use super::LinuxType;

    #[test]
    fn amazon_path() {
        let version = Version::parse("3.4.6").unwrap();

        assert_eq!(
            vec!["x86_64", "amazon", "3.4.6"],
            LinuxType::Amazon.url_path(&version)
        );
    }

    #[test]
    fn debian7_path() {
        let version = Version::parse("3.4.6").unwrap();

        assert_eq!(
            vec!["x86_64", "debian71", "3.4.6"],
            LinuxType::Debian7.url_path(&version)
        );
    }

    #[test]
    fn debian8_path() {
        let version = Version::parse("3.4.6").unwrap();

        assert_eq!(
            vec!["x86_64", "debian81", "3.4.6"],
            LinuxType::Debian8.url_path(&version)
        );
    }

    #[test]
    fn legacy_path() {
        let version = Version::parse("3.4.6").unwrap();

        assert_eq!(
            vec!["x86_64", "3.4.6"],
            LinuxType::Legacy.url_path(&version)
        );
    }

    #[test]
    fn rhel6_path() {
        let version = Version::parse("3.4.6").unwrap();

        assert_eq!(
            vec!["x86_64", "rhel62", "3.4.6"],
            LinuxType::Rhel6.url_path(&version)
        );
    }

    #[test]
    fn rhel7_path() {
        let version = Version::parse("3.4.6").unwrap();

        assert_eq!(
            vec!["x86_64", "rhel70", "3.4.6"],
            LinuxType::Rhel7.url_path(&version)
        );
    }

    #[test]
    fn suse11_path() {
        let version = Version::parse("3.4.6").unwrap();

        assert_eq!(
            vec!["x86_64", "suse11", "3.4.6"],
            LinuxType::Suse11.url_path(&version)
        );
    }

    #[test]
    fn suse12_path() {
        let version = Version::parse("3.4.6").unwrap();

        assert_eq!(
            vec!["x86_64", "suse12", "3.4.6"],
            LinuxType::Suse12.url_path(&version)
        );
    }

    #[test]
    fn ubuntu1204_path() {
        let version = Version::parse("3.4.6").unwrap();

        assert_eq!(
            vec!["x86_64", "ubuntu1204", "3.4.6"],
            LinuxType::Ubuntu1204.url_path(&version)
        );
    }

    #[test]
    fn ubuntu1404_path() {
        let version = Version::parse("3.4.6").unwrap();

        assert_eq!(
            vec!["x86_64", "ubuntu1404", "3.4.6"],
            LinuxType::Ubuntu1404.url_path(&version)
        );
    }

    #[test]
    fn ubuntu1604_arm_path() {
        let version = Version::parse("3.4.6").unwrap();

        assert_eq!(
            vec!["arm64", "ubuntu1604", "3.4.6"],
            LinuxType::Ubuntu1604(Architecture::Arm).url_path(&version)
        );
    }

    #[test]
    fn ubuntu1604_x86_64_path() {
        let version = Version::parse("3.4.6").unwrap();

        assert_eq!(
            vec!["x86_64", "ubuntu1604", "3.4.6"],
            LinuxType::Ubuntu1604(Architecture::X86_64).url_path(&version)
        );
    }
}
