use lazy_static::lazy_static;
use semver::Version;

lazy_static! {
    static ref FIRST_MACOS_VERSION: Version = version!(3, 5, 4);
    static ref LAST_MACOS_VERSION: Version = version!(3, 6, 0);
    static ref NEW_MACOS_VERSION: Version = version!(4, 1, 1);
}

fn name_uses_macos(version: &Version) -> bool {
    *version >= *FIRST_MACOS_VERSION && *version < *LAST_MACOS_VERSION
}

fn url_uses_macos(version: &Version) -> bool {
    *version >= *NEW_MACOS_VERSION
}

#[derive(Debug)]
pub struct Url {
    base: String,
    filename: String,
    dirname: String,
}

impl Url {
    pub fn filename(&self) -> String {
        self.filename.clone()
    }

    pub fn dirname(&self) -> String {
        self.dirname.clone()
    }
}

impl From<Url> for String {
    fn from(url: Url) -> Self {
        format!("{}/{}", url.base, url.filename)
    }
}

#[derive(Debug)]
pub struct UrlBuilder<'a> {
    os: &'a str,
    distro: Vec<String>,
    extension: &'a str,
    version: &'a Version,
}

const SCHEME: &str = "https";
const DOMAIN: &str = "fastdl.mongodb.org";

impl<'a> UrlBuilder<'a> {
    pub fn new(os: &'a str, extension: &'a str, version: &'a Version) -> UrlBuilder<'a> {
        Self {
            os,
            distro: Vec::new(),
            extension,
            version,
        }
    }

    pub fn add_distro_path_item(&mut self, item: String) {
        self.distro.push(item)
    }

    pub fn build(mut self) -> Url {
        let base = format!("{}://{}/{}", SCHEME, DOMAIN, self.os);

        let mut filename = String::new();
        let mut dirname = String::new();

        if url_uses_macos(self.version) && self.distro[1..=2] == ["osx", "ssl"] {
            // This is inefficient, but there are only a handful of elements, so we don't care.
            let mut replacement = vec!["mongodb".to_string(), "macos".to_string()];
            replacement.extend(self.distro.split_off(3));

            self.distro = replacement;
        }

        for (i, mut item) in self.distro.into_iter().enumerate() {
            if i != 0 {
                filename.push('-');
            }

            filename.push_str(&item);

            if item == "ssl" {
                continue;
            }

            if item == "osx" && name_uses_macos(self.version) {
                item = "macOS".to_string();
            }

            if i != 0 {
                dirname.push('-');
            }

            dirname.push_str(&item);
        }

        filename.push('.');
        filename.push_str(self.extension);

        Url {
            base,
            filename,
            dirname,
        }
    }
}
