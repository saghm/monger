use clap::ArgMatches;
use monger_core::{error::Result, process::ChildType, Monger};

use crate::util::file_exists_in_path;

pub fn dispatch(args: &ArgMatches) -> Result<()> {
    let monger = Monger::new()?;

    match args.subcommand() {
        ("delete", Some(m)) => delete(&monger, m),
        ("get", Some(m)) => get(&monger, m),
        ("prune", _) => prune(&monger),
        ("list", _) => list(&monger),
        ("run", Some(m)) => run(&monger, m),
        ("start", Some(m)) => start(&monger, m),
        _ => invariant!("subcommand must be provided with requisite args"),
    }
}

fn delete(monger: &Monger, matches: &ArgMatches) -> Result<()> {
    match matches.value_of("ID") {
        Some(version) => monger.delete_mongodb_version(version),
        None => invariant!("`monger delete` must supply ID"),
    }
}

fn get(monger: &Monger, matches: &ArgMatches) -> Result<()> {
    let version_str = match matches.value_of("VERSION") {
        Some(version) => version,
        None => invariant!("`monger get` must supply version"),
    };

    let force = matches.is_present("force");
    let os = matches.value_of("os");
    let id = matches.value_of("id");
    monger.download_mongodb_version(version_str, force, os, id)
}

fn list(monger: &Monger) -> Result<()> {
    let mut versions: Vec<_> = monger
        .list_versions()?
        .into_iter()
        .map(|s| s.to_string_lossy().into_owned())
        .collect();
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

fn prune(monger: &Monger) -> Result<()> {
    monger.prune()
}

fn run(monger: &Monger, matches: &ArgMatches) -> Result<()> {
    let version = matches
        .value_of("ID")
        .unwrap_or_else(|| invariant!("`monger run` must provide ID"));

    let bin = matches
        .value_of("BIN")
        .unwrap_or_else(|| invariant!("`monger run` must provide binary"));

    let args = matches.values_of("BIN_ARGS").unwrap_or_default();

    monger.command(bin, args, version, ChildType::Exec)
}

fn start(monger: &Monger, matches: &ArgMatches) -> Result<()> {
    let version = matches
        .value_of("ID")
        .unwrap_or_else(|| invariant!("`monger run` must provide ID"));

    let args = matches.values_of("MONGOD_ARGS").unwrap_or_default();

    monger.start_mongod(args.map(Into::into), version, ChildType::Exec)
}
