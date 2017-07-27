error_chain! {
    foreign_links {
        Http(::reqwest::Error);
        Io(::std::io::Error);
        OsRelease(::rs_release::OsReleaseError);
        SemVer(::semver::SemVerError);
    }

    errors {
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
