#![recursion_limit = "128"]

#[macro_use]
extern crate clap;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate lazy_static;

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

use crate::{dispatch::dispatch, error::Result, os::OS_NAMES};

quick_main!(|| -> Result<i32> {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about(crate_description!())
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("delete")
                .about("deletes an installed MongoDB version")
                .arg(
                    Arg::with_name("VERSION")
                        .help("the MongoDB version to delete")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("fetch")
                .about("manually downloads a MongoDB version for a specific OS")
                .arg(
                    Arg::with_name("OS")
                        .possible_values(&OS_NAMES)
                        .help("The OS version to download.")
                        .required(true),
                )
                .arg(
                    Arg::with_name("VERSION")
                        .required(true)
                        .help("the MongoDB version to download"),
                ),
        )
        .subcommand(
            SubCommand::with_name("get")
                .about("downloads a MongoDB version")
                .arg(
                    Arg::with_name("VERSION")
                        .help("the MongoDB version to download")
                        .required(true),
                )
                .arg(
                    Arg::with_name("force")
                        .help("download the MongoDB version even if it already is installed")
                        .short("f")
                        .long("force"),
                ),
        )
        .subcommand(SubCommand::with_name("list").about("lists installed MongoDB versions"))
        .subcommand(SubCommand::with_name("prune").about(
            "deletes versions of MongoDB where a newer stable version of the same minor version \
             is installed",
        ))
        .subcommand(
            SubCommand::with_name("run")
                .about("run a binary of a downloaded MongoDB version")
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
        .subcommand(
            SubCommand::with_name("start")
                .about("start an installed mongod")
                .arg(
                    Arg::with_name("VERSION")
                        .help("the mongod version to start")
                        .required(true),
                )
                .arg(
                    Arg::with_name("MONGOD_ARGS")
                        .help("extra arguments for the mongod being run")
                        .multiple(true)
                        .last(true),
                ),
        )
        .get_matches();

    dispatch(&matches)?;
    Ok(0)
});
