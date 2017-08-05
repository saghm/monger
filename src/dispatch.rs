use clap::ArgMatches;

use error::Result;
use monger::Monger;

pub fn dispatch(args: ArgMatches) -> Result<()> {
    let monger = Monger::new()?;

    match args.subcommand() {
        ("get", Some(m)) => get(monger, m),
        ("run", Some(m)) => run(monger, m),
        _ => invariant!("subcommand must be provided with requisite args"),
    }
}

fn get(monger: Monger, matches: &ArgMatches) -> Result<()> {
    match matches.value_of("VERSION") {
        Some(version) => monger.download_mongodb_version(version),
        None => invariant!("`monger get` must supply version"),
    }
}

fn run(monger: Monger, matches: &ArgMatches) -> Result<()> {
    let version = matches.value_of("VERSION").unwrap_or_else(|| {
        invariant!("`monger run` must provide version")
    });

    let bin = matches.value_of("BIN").unwrap_or_else(|| {
        invariant!("`monger run` must provide binary")
    });

    let args = matches.values_of("BIN_ARGS").unwrap_or_default();

    monger.exec(bin, args, &version)
}
