use std::fs::metadata;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::str::FromStr;

use regex::Regex;
use semver::Version;

static EXECUTABLE_BITS: u32 = 0b0_0100_1001;

lazy_static! {
    static ref VERSION_WITHOUT_PATCH: Regex = Regex::new(r"^(\d+)\.(\d+)$").unwrap();
}

pub enum FileExtension {
    Msi,
    Tgz,
}

impl FileExtension {
    pub fn name(&self) -> &'static str {
        match *self {
            FileExtension::Msi => "msi",
            FileExtension::Tgz => "tgz",
        }
    }
}

#[macro_export]
macro_rules! try_option {
    ($opt:expr) => {
        match $opt {
            Some(val) => val,
            None => return None,
        }
    };
}

#[inline]
pub fn get_from_str<T: FromStr>(s: &str) -> Option<T> {
    FromStr::from_str(s).ok()
}

#[macro_export]
macro_rules! invariant {
    ($msg:expr) => {
        panic!($msg)
    };
}

macro_rules! version {
    ($major:expr, $minor:expr, $patch:expr) => {{
        ::semver::Version {
            major: $major,
            minor: $minor,
            patch: $patch,
            pre: Vec::new(),
            build: Vec::new(),
        }
    }};
}

#[inline]
fn is_executable(mode: u32) -> bool {
    mode & EXECUTABLE_BITS == EXECUTABLE_BITS
}

pub fn file_exists_in_path<P: AsRef<Path>>(file: P) -> bool {
    env!("PATH").split(':').any(|dir| {
        let path = Path::new(dir).join(file.as_ref());

        let data = match metadata(path) {
            Ok(m) => m,
            Err(_) => return false,
        };

        if !data.is_file() {
            return false;
        }

        is_executable(data.permissions().mode())
    })
}

pub fn select_newer_version(existing: Option<Version>, found: Version) -> Version {
    if let Some(version) = existing {
        if version > found {
            return version;
        }
    }

    found
}

pub fn parse_major_minor_version(version: &str) -> Option<(u64, u64)> {
    VERSION_WITHOUT_PATCH
        .captures(version)
        .map(|c| (c[1].parse().unwrap(), c[2].parse().unwrap()))
}
