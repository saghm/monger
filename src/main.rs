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
mod monger;
mod os;
mod process;
mod url;

use error::Result;
use monger::Monger;

quick_main!(run);

fn run() -> Result<i32> {
    let monger = Monger::new()?;
    monger.download_mongodb_version("3.4.6")?;
    monger.download_mongodb_version("3.2.16")?;

    Ok(0)
}
