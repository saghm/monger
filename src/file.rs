use std::fs::OpenOptions;
use std::io::Write;

use error::Result;

pub fn write_file(path: &str, data: &mut [u8]) -> Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)?;

    file.write_all(data).map_err(From::from)
}