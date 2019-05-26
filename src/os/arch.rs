#[derive(Clone, Copy, Debug)]
pub enum Architecture {
    #[allow(dead_code)]
    Arm,

    #[allow(non_camel_case_types)]
    X86_64,
}

impl Architecture {
    pub fn name(self) -> &'static str {
        match self {
            Architecture::Arm => "arm64",
            Architecture::X86_64 => "x86_64",
        }
    }
}
