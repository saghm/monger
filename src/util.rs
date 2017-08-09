use std::str::FromStr;

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
            None => return None
        }
    };
}

#[inline]
pub fn get_from_str<T: FromStr>(s: &str) -> Option<T> {
    FromStr::from_str(s).ok()
}

#[macro_export]
macro_rules! invariant {
    ($msg:expr) => { panic!($msg) };
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
