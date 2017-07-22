#[derive(Clone, Copy, Debug)]
pub enum Architecture {
    Arm,
    X86_64,
}

impl Architecture {
    pub fn name(&self) -> &'static str {
        match *self {
            Architecture::Arm => "arm64",
            Architecture::X86_64 => "x86_64",
        }
    }
}
