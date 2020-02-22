use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error(
        "Unable to find {binary} to execute. Run `monger get {version}` and try again if you're \
         sure the version and binary name are correct"
    )]
    BinaryNotFound { binary: String, version: String },

    #[error("`{command}` command failed")]
    FailedSubprocess {
        command: String,
        exit_code: Option<i32>,
    },

    #[error("An HTTP error occurred: {inner}")]
    Http {
        #[from]
        inner: reqwest::Error,
    },

    #[error("HTML response from {url} did not match expected structure")]
    InvalidHtml { url: String },

    #[error("MongoDB version {version} does not exist")]
    InvalidVersion { version: String },

    #[error("An I/O error occurred: {inner}")]
    Io {
        #[from]
        inner: std::io::Error,
    },

    #[error("Unable to determine the OS release version")]
    OsRelease {
        #[from]
        inner: rs_release::OsReleaseError,
    },

    #[error("Unable to parse semantic version")]
    SemVer {
        #[from]
        inner: semver::SemVerError,
    },

    #[error("Unable to convert HTTP header to string: {inner}")]
    ToStr {
        #[from]
        inner: reqwest::header::ToStrError,
    },

    #[error("Unable to find home directory")]
    UnknownHomeDirectory,

    #[error("Unable to identify operating system")]
    UnknownOs,

    #[error("{os_name} is unsupported")]
    UnsupportedOs { os_name: String },

    #[error("Unable to find version {version}")]
    VersionNotFound { version: String },
}
