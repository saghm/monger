#[derive(Debug)]
pub struct Url {
    base: String,
    filename: String,
}

impl Url {
    pub fn filename(&self) -> String {
        self.filename.clone()
    }
}

impl From<Url> for String {
    fn from(url: Url) -> Self {
        format!("{}/{}", url.base, url.filename)
    }
}

#[derive(Debug)]
pub struct UrlBuilder {
    os: String,
    distro: Vec<String>,
    extension: String,
}

const SCHEME: &str = "https";
const DOMAIN: &str = "fastdl.mongodb.org";

impl UrlBuilder {
    pub fn new(os: &str, extension: &str) -> Self {
        Self {
            os: os.to_string(),
            distro: Vec::new(),
            extension: extension.to_string(),
        }
    }

    pub fn add_distro_path_item(&mut self, item: String) {
        self.distro.push(item)
    }

    pub fn build(self) -> Url {
        let base = format!("{}://{}/{}", SCHEME, DOMAIN, self.os);

        let mut filename = String::new();

        for (i, item) in self.distro.iter().enumerate() {
            if i != 0 {
                filename.push_str("-");
            }

            filename.push_str(&item);
        }

        filename.push_str(".");
        filename.push_str(&self.extension);

        Url { base, filename }
    }
}
