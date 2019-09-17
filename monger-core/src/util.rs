use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;
use semver::Version;

use crate::error::{Error, Result};

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

#[inline]
pub fn get_from_str<T: FromStr>(s: &str) -> Option<T> {
    FromStr::from_str(s).ok()
}

#[macro_export]
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

pub fn parse_version(version: &str) -> Result<Version> {
    Version::parse(version).map_err(|_| {
        Error::VersionNotFound {
            version: version.into(),
        }
        .into()
    })
}
