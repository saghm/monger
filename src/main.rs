#[macro_use]
extern crate error_chain;
extern crate reqwest;
extern crate rs_release;
extern crate semver;

#[macro_use]
mod util;

//mod client;
mod error;
//mod file;
mod os;
mod url;

use semver::Version;

// use client::HttpClient;
use error::Result;
//use file::write_file;
use os::OperatingSystem;

fn run() -> Result<()> {
    let version = Version::parse("3.4.6")?;
    let url = OperatingSystem::get()?.download_url(&version);
    //    let client = HttpClient::new()?;
    //    let file = url.filename();
    let url: String = url.into();

    println!("downloading {}...", url);
    //    let data = client.download_file(&url)?;
    //
    //    println!("writing {}...", file);
    //    write_file(&file, &data[..])?;

    Ok(())
}

fn main() {
    run().unwrap()
}
