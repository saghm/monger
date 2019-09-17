use semver::Version;

use super::arch::Architecture;

#[derive(Clone, Debug)]
pub enum WindowsType {
    Server2008,
    Server2008R2,
    Server2008R2Ssl,
}

impl WindowsType {
    #[inline]
    fn architecture(&self) -> Architecture {
        Architecture::X86_64
    }

    fn name(&self) -> Option<&'static str> {
        match *self {
            WindowsType::Server2008R2 | WindowsType::Server2008R2Ssl => Some("2008plus"),
            WindowsType::Server2008 => None,
        }
    }

    pub fn url_path(&self, version: &Version) -> Vec<String> {
        let mut path = vec![self.architecture().name().to_string()];

        if let Some(name) = self.name() {
            path.push(name.to_string());
        }

        if let WindowsType::Server2008R2Ssl = *self {
            path.push("ssl".to_string());
        }

        path.push(format!("{}", version));
        path.push("signed".to_string());
        path
    }
}

#[cfg(test)]
mod tests {
    use super::WindowsType;

    #[test]
    fn server2008_path() {
        let version = version!(3, 4, 6);

        assert_eq!(
            vec!["x86_64", "3.4.6", "signed"],
            WindowsType::Server2008.url_path(&version)
        );
    }

    #[test]
    fn server2008_r2_path() {
        let version = version!(3, 4, 6);

        assert_eq!(
            vec!["x86_64", "2008plus", "3.4.6", "signed"],
            WindowsType::Server2008R2.url_path(&version)
        );
    }

    #[test]
    fn server2008_r2_ssl_path() {
        let version = version!(3, 4, 6);

        assert_eq!(
            vec!["x86_64", "2008plus", "ssl", "3.4.6", "signed"],
            WindowsType::Server2008R2Ssl.url_path(&version)
        );
    }
}
