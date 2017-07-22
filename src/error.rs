error_chain! {
    foreign_links {
        Http(::reqwest::Error);
        Io(::std::io::Error);
    }
}
