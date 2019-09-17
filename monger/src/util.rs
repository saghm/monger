use std::{fs::metadata, os::unix::fs::PermissionsExt, path::Path};

const EXECUTABLE_BITS: u32 = 0b0_0100_1001;

#[macro_export]
macro_rules! invariant {
    ($msg:expr) => {
        panic!($msg)
    };
}

#[inline]
fn is_executable(mode: u32) -> bool {
    mode & EXECUTABLE_BITS == EXECUTABLE_BITS
}

pub fn file_exists_in_path<P: AsRef<Path>>(file: P) -> bool {
    env!("PATH").split(':').any(|dir| {
        let path = Path::new(dir).join(file.as_ref());

        let data = match metadata(path) {
            Ok(m) => m,
            Err(_) => return false,
        };

        if !data.is_file() {
            return false;
        }

        is_executable(data.permissions().mode())
    })
}
