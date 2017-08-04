#[macro_use]
extern crate error_chain;

extern crate reqwest;
extern crate rs_release;
extern crate semver;

#[macro_use]
mod util;

mod client;
mod error;
mod fs;
mod os;
mod process;
mod url;

use semver::Version;

use client::HttpClient;
use error::Result;
use fs::Fs;
use os::OperatingSystem;

quick_main!(run);

fn run() -> Result<i32> {
    let version = Version::parse("3.4.6")?;
    let url = OperatingSystem::get()?.download_url(&version);
    let client = HttpClient::new()?;
    let file = url.filename();
    let url: String = url.into();

    println!("downloading {}...", url);
    let data = client.download_file(&url)?;

    println!("writing {}...", file);
    let fs = Fs::default()?;
    fs.write_file(&file, &data[..])?;

    println!("decompressing {}...", file);
    fs.decompress(&file)?;

    Ok(0)
}
