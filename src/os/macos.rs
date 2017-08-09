use semver::Version;

use super::arch::Architecture;

#[derive(Debug)]
pub enum MacOsType {
    NonSsl,
    Ssl,
}

impl MacOsType {
    #[inline]
    fn architecture(&self) -> Architecture {
        Architecture::X86_64
    }

    pub fn url_path(&self, version: &Version) -> Vec<String> {
        let mut path = Vec::new();

        if let MacOsType::Ssl = *self {
            path.push("ssl".to_string());
        }

        path.push(self.architecture().name().to_string());
        path.push(format!("{}", version));
        path
    }
}

#[cfg(test)]
mod tests {
    use super::MacOsType;

    #[test]
    fn nonssl_path() {
        let version = version!(3, 4, 6);

        assert_eq!(
            vec!["x86_64", "3.4.6"],
            MacOsType::NonSsl.url_path(&version)
        );
    }

    #[test]
    fn ssl_path() {
        let version = version!(3, 4, 6);

        assert_eq!(
            vec!["ssl", "x86_64", "3.4.6"],
            MacOsType::Ssl.url_path(&version)
        );
    }
}
