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
