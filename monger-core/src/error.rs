use err_derive::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error(
        display = "Unable to find binary to execute. Run `monger get {}` and try again if you're sure the version and binary name are correct",
        version
    )]
    BinaryNotFound { binary: String, version: String },

    #[error(display = "`{}` command failed", command)]
    FailedSubprocess {
        command: String,
        exit_code: Option<i32>,
    },

    #[error(display = "An HTTP error occurred: {}", inner)]
    Http { inner: reqwest::Error },

    #[error(
        display = "HTML response from {} did not match expected structure",
        url
    )]
    InvalidHtml { url: String },

    #[error(display = "MongoDB version {} does not exist", version)]
    InvalidVersion { version: String },

    #[error(display = "An I/O error occurred: {}", inner)]
    Io { inner: std::io::Error },

    #[error(display = "Unable to determine the OS release version")]
    OsRelease { inner: rs_release::OsReleaseError },

    #[error(display = "Unable to parse semantic version")]
    SemVer { inner: semver::SemVerError },

    #[error(display = "Unable to convert HTTP header to string: {}", inner)]
    ToStr { inner: reqwest::header::ToStrError },

    #[error(display = "Unable to find home directory")]
    UnknownHomeDirectory,

    #[error(display = "Unable to identify operating system")]
    UnknownOs,

    #[error(display = "{} is unsupported", os_name)]
    UnsupportedOs { os_name: String },

    #[error(display = "Unable to find version {}", version)]
    VersionNotFound { version: String },
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self::Http { inner: err }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::Io { inner: err }
    }
}

impl From<rs_release::OsReleaseError> for Error {
    fn from(err: rs_release::OsReleaseError) -> Self {
        Self::OsRelease { inner: err }
    }
}

impl From<semver::SemVerError> for Error {
    fn from(err: semver::SemVerError) -> Self {
        Self::SemVer { inner: err }
    }
}

impl From<reqwest::header::ToStrError> for Error {
    fn from(err: reqwest::header::ToStrError) -> Self {
        Self::ToStr { inner: err }
    }
}
