use std::io::Read;

use reqwest::{Client, ClientBuilder, Response};

use crate::error::{Error, Result};

pub struct HttpClient {
    client: Client,
}

impl HttpClient {
    pub fn new() -> Result<Self> {
        Ok(Self {
            client: ClientBuilder::new().gzip(false).build()?,
        })
    }

    pub fn get(&self, url: &str) -> Result<Response> {
        let response = self.client.get(url).send()?;
        Ok(response)
    }

    pub fn download_file(&self, url: &str, version: &str) -> Result<Vec<u8>> {
        println!("downloading {}...", url);
        let mut data = Vec::new();
        let mut response = self.client.get(url).send()?;

        if !response.status().is_success() {
            return Err(Error::InvalidVersion {
                version: version.into(),
            });
        }

        response.read_to_end(&mut data)?;

        Ok(data)
    }
}

#[cfg(test)]
mod tests {
    use super::HttpClient;

    #[test]
    fn download_test() {
        let client = HttpClient::new().unwrap();
        let data = client
            .download_file("https://httpbin.org/robots.txt", "null")
            .unwrap();
        let expected = "User-agent: *\nDisallow: /deny\n".to_string();

        assert_eq!(expected.into_bytes(), data);
    }
}
