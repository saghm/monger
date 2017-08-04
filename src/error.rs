error_chain! {
    foreign_links {
        Http(::reqwest::Error);
        Io(::std::io::Error);
        OsRelease(::rs_release::OsReleaseError);
        SemVer(::semver::SemVerError);
    }

    errors {
        FailedSubprocess(cmd: String, exit_code: Option<i32>) {
            description("a subprocess for monger has failed")
            display("{}: {}",
                    cmd,
                    exit_code.map(|i| format!("{}", i)).unwrap_or("unknown exit code".to_string()))
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
    }
}
