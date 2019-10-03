#![recursion_limit = "128"]

#[macro_use]
extern crate clap;

#[macro_use]
mod util;
mod dispatch;

use clap::{App, AppSettings, Arg, SubCommand};
use monger_core::{error::Result, os::OS_NAMES};

use crate::dispatch::dispatch;

fn main() -> Result<()> {
    #[allow(deprecated)]
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about(crate_description!())
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("clear")
                .about("clear the database files for an installed MongoDB version")
                .arg(
                    Arg::with_name("ID")
                        .help("the ID of the MongoDB version whose files should be cleared")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("delete")
                .about("deletes an installed MongoDB version")
                .arg(
                    Arg::with_name("ID")
                        .help("the ID of the MongoDB version to delete")
                        .required(true),
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
                )
                .arg(
                    Arg::with_name("os")
                        .help("the OS version to download.")
                        .long("os")
                        .takes_value(true)
                        .possible_values(&OS_NAMES),
                )
                .arg(
                    Arg::with_name("id")
                        .help(
                            "specify a unique identifier for the MongoDB version being \
                             downloaded; if not specified, it will default to the version string \
                             (i,e, 'x.y.z')",
                        )
                        .long("id")
                        .takes_value(true),
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
                    Arg::with_name("ID")
                        .help("the ID of the MongoDB version of the binary being run")
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
                    Arg::with_name("ID")
                        .help("the ID of the mongod version to start")
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
    Ok(())
}
