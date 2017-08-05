#[macro_use]
extern crate clap;
#[macro_use]
extern crate error_chain;
extern crate reqwest;
extern crate rs_release;
extern crate semver;

#[macro_use]
mod util;

mod client;
mod dispatch;
mod error;
mod fs;
mod monger;
mod os;
mod process;
mod url;

use clap::{App, AppSettings, Arg, SubCommand};

use dispatch::dispatch;
use error::Result;

quick_main!(run);

fn run() -> Result<i32> {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about(crate_description!())
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("delete")
                .help("deletes an installed MongoDB version")
                .arg(
                    Arg::with_name("VERSION")
                        .help("the MongoDB version to delete")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("get")
                .help("downloads a MongoDB version")
                .arg(
                    Arg::with_name("VERSION")
                        .help("the MongoDB version to download")
                        .required(true),
                )
                .arg(
                    Arg::with_name("force")
                        .help(
                            "download the MongoDB version even if it already is installed",
                        )
                        .short("f")
                        .long("force"),
                ),
        )
        .subcommand(
            SubCommand::with_name("run")
                .help("run a binary of a downloaded MongoDB version")
                .arg(
                    Arg::with_name("VERSION")
                        .help("the MongoDB version of the binary being run")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("BIN")
                        .help("the MongoDB binary to run")
                        .required(true)
                        .index(2),
                )
                .arg(
                    Arg::with_name("BIN_ARGS")
                        .help("arguments for the MongoDB binary being run")
                        .multiple(true)
                        .last(true),
                ),
        )
        .get_matches();

    dispatch(matches)?;
    Ok(0)
}
