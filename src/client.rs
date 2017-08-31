use std::io::Read;

use reqwest::{Client, ClientBuilder};
use serde_json::{self, Value};

use error::Result;

pub struct HttpClient {
    client: Client,
}

impl HttpClient {
    pub fn new() -> Result<Self> {
        Ok(Self { client: ClientBuilder::new()?.gzip(false).build()? })
    }

    pub fn get_json(&self, url: &str) -> Result<Value> {
        let response = self.client.get(url)?.send()?;
        let value = serde_json::from_reader(response)?;

        Ok(value)
    }

    pub fn download_file(&self, url: &str) -> Result<Vec<u8>> {
        println!("downloading {}...", url);
        let mut data = Vec::new();
        let mut response = self.client.get(url)?.send()?;
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
            .download_file("https://httpbin.org/robots.txt")
            .unwrap();
        let expected = "User-agent: *\nDisallow: /deny\n".to_string();

        assert_eq!(expected.into_bytes(), data);
    }
}
