#![recursion_limit = "128"]
#![allow(dead_code)]

#[macro_use]
mod util;
mod dispatch;

use anyhow::Result;
use monger_core::os::OS_NAMES;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about, author)]
enum Options {
    /// clear the database files for an installed MongoDB version
    Clear {
        /// the ID of the MongoDB version whose files should be cleared
        #[structopt(name = "ID")]
        id: String,
    },

    /// deletes an installed MongoDB version
    Delete {
        /// the ID of the MongoDB version to delete
        #[structopt(name = "ID")]
        id: String,
    },

    /// manages the default arguments used when starting a mongod
    Defaults(Defaults),

    /// downloads a MongoDB version from a given URL
    Download {
        /// the URL to download from
        #[structopt(name = "URL")]
        url: String,

        /// specify a unique identifier for the MongoDB version being downloaded
        #[structopt(long)]
        id: String,

        /// download the MongoDB version even if it already is installed
        #[structopt(long, short)]
        force: bool,
    },

    /// downloads a MongoDB version
    Get {
        /// the MongoDB version to download
        #[structopt(name = "VERSION")]
        version: String,

        /// download the MongoDB version even if it already is installed
        #[structopt(long, short)]
        force: bool,

        /// the OS version to download
        #[structopt(long, possible_values(&OS_NAMES))]
        os: Option<String>,

        /// specify a unique identifier for the MongoDB version being downloaded; if not specified,
        /// it will default to the version string (i,e, 'x.y.z')
        #[structopt(long)]
        id: Option<String>,
    },

    /// lists installed MongoDB versions
    List,

    /// deletes versions of MongoDB where a newer stable version of the same minor version is
    /// installed
    Prune,

    /// run a binary of a downloaded MongoDB version
    Run {
        /// the ID of the MongoDB version of the binary being run
        #[structopt(name = "ID")]
        id: String,

        /// the MongoDB binary to run
        #[structopt(name = "BIN")]
        bin: String,

        /// arguments for the MongoDB binary being run
        #[structopt(name = "BIN_ARGS", last(true))]
        bin_args: Vec<String>,
    },

    /// updates monger to the latest version
    SelfUpdate,

    /// start an installed mongod
    Start {
        /// the ID of the mongod version to start
        #[structopt(name = "ID")]
        id: String,

        /// extra arguments for the mongod being run
        #[structopt(name = "MONGODB_ARGS", last(true))]
        mongod_args: Vec<String>,
    },
}

#[derive(Debug, StructOpt)]
enum Defaults {
    /// clears the previously set default arguments
    Clear,

    /// prints the default arguments used when starting a mongod
    Get,

    /// sets the default arguments used when starting a mongod
    Set {
        #[structopt(name = "ARGS", last(true))]
        args: Vec<String>,
    },
}

fn main() -> Result<()> {
    Options::from_args().dispatch()
}
