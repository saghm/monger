#[macro_use]
extern crate error_chain;
extern crate reqwest;
extern crate semver;

mod client;
mod error;
mod file;
mod os;
mod url;
mod util;

use client::HttpClient;
use file::write_file;

fn main() {
    let client = HttpClient::new().unwrap();
    let mut data = client.download_file("https://httpbin.org/ip").unwrap();
    write_file("ip.json", &mut data[..]).unwrap();
}
