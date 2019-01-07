error_chain! {
    foreign_links {
        Clap(::clap::Error);
        Error(::hyperx::Error);
        Http(::reqwest::Error);
        Io(::std::io::Error);
        Json(::serde_json::Error);
        OsRelease(::rs_release::OsReleaseError);
        SemVer(::semver::SemVerError);
        ToStr(::reqwest::header::ToStrError);
    }

    errors {
       BinaryNotFound(bin: String, version: String)  {
            description("unable to find binary to run")
            display("Unable to find `{}` version {}\n\nRun `monger get {}` and try again if you're \
                    sure the version and binary name are correct",
                    bin,
                    version,
                    version,
            )
        }

        FailedSubprocess(cmd: String, exit_code: Option<i32>) {
            description("a subprocess for monger has failed")
            display("{}: {}",
                    cmd,
                    exit_code
                        .map(|i| format!("{}", i))
                        .unwrap_or_else(|| "unknown exit code".to_string()))
        }

        InvalidHtml(url: String) {
            description("HTML did not match expected structure")
            display("HTML response from {} did not match expected structure", url)
        }

        InvalidVersion(version: String) {
            description("The provided version of MongoDB does not exist")
            display("MongoDB version {} does not exist", version)
        }

        UnknownHomeDirectory {
            description("unable to find home directory")
            display("unable to find home directory")
        }

        UnknownOs {
            description("unable to identify operating system")
            display("unable to identify operating system")
        }

        UnsupportedOs(t: String) {
            description("unsupported operating system")
            display("{} is currently unsupported", t)
        }

        VersionNotFound(t: String) {
            description("unable to find specified version")
            display("unable to find version: {}", t)
        }
    }
}
