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

    pub fn build(self) -> String {
        let mut url = format!("{}://{}/{}/", SCHEME, DOMAIN, self.os);

        for (i, item) in self.distro.into_iter().enumerate() {
            if i != 0 {
                url.push_str("-");
            }

            url.push_str(&item);
        }

        url.push_str(".");
        url.push_str(&self.extension);
        url
    }
}
