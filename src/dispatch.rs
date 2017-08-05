use clap::ArgMatches;
use semver::Version;

use error::Result;
use monger::Monger;

pub fn dispatch(args: ArgMatches) -> Result<()> {
    let monger = Monger::new()?;

    match args.subcommand() {
        ("delete", Some(m)) => delete(&monger, m),
        ("list", _) => list(&monger),
        ("get", Some(m)) => get(&monger, m),
        ("run", Some(m)) => run(&monger, m),
        _ => invariant!("subcommand must be provided with requisite args"),
    }
}

fn delete(monger: &Monger, matches: &ArgMatches) -> Result<()> {
    match matches.value_of("VERSION") {
        Some(version) => monger.delete_mongodb_version(version),
        None => invariant!("`monger delete` must supply version"),
    }
}

fn get(monger: &Monger, matches: &ArgMatches) -> Result<()> {
    if matches.is_present("force") {
        delete(monger, matches)?;
    }

    match matches.value_of("VERSION") {
        Some(version) => monger.download_mongodb_version(version),
        None => invariant!("`monger get` must supply version"),
    }
}

fn list(monger: &Monger) -> Result<()> {
    let mut versions: Vec<_> = monger
        .list_versions()?
        .into_iter()
        .map(|s| Version::parse(s.to_string_lossy().as_ref()).map_err(From::from))
        .collect::<Result<_>>()?;
    versions.sort();

    println!("installed versions:");

    for version in versions {
        println!("    {}", version);
    }

    Ok(())
}

fn run(monger: &Monger, matches: &ArgMatches) -> Result<()> {
    let version = matches.value_of("VERSION").unwrap_or_else(|| {
        invariant!("`monger run` must provide version")
    });

    let bin = matches.value_of("BIN").unwrap_or_else(|| {
        invariant!("`monger run` must provide binary")
    });

    let args = matches.values_of("BIN_ARGS").unwrap_or_default();

    monger.exec(bin, args, &version)
}
