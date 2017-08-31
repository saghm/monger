use clap::ArgMatches;
use semver::Version;

use error::Result;
use monger::Monger;
use util::file_exists_in_path;

pub fn dispatch(args: &ArgMatches) -> Result<()> {
    let monger = Monger::new()?;

    match args.subcommand() {
        ("delete", Some(m)) => delete(&monger, m),
        ("list", _) => list(&monger),
        ("get", Some(m)) => get(&monger, m),
        ("run", Some(m)) => run(&monger, m),
        ("start", Some(m)) => start(&monger, m),
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
        .map(|s| {
            Version::parse(s.to_string_lossy().as_ref())
                .map(|v| v.to_string())
                .map_err(From::from)
        })
        .collect::<Result<_>>()?;
    versions.sort();

    if file_exists_in_path("mongod") {
        versions.push("system".to_string());
    }

    print!("installed versions:");

    if versions.is_empty() {
        println!(" none");
    } else {
        println!();

        for version in versions {
            println!("    {}", version);
        }
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

    monger.exec(bin, args, version)
}

fn start(monger: &Monger, matches: &ArgMatches) -> Result<()> {
    let version = matches.value_of("VERSION").unwrap_or_else(|| {
        invariant!("`monger run` must provide version")
    });

    let args = matches.values_of("MONGOD_ARGS").unwrap_or_default();

    monger.start_mongod(args.map(Into::into), version)
}
