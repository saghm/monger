use std::env::home_dir;
use std::fs::{create_dir_all, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

use error::{ErrorKind, Result};
use process::run_command;

const DEFAULT_MONGER_DIRECTORY: &str = "code/projects/monger/ds";

pub struct Fs {
    path: PathBuf,
}

impl Fs {
    pub fn new(monger_dir: &str) -> Result<Self> {
        match home_dir() {
            Some(mut path) => {
                path.push(monger_dir);
                Ok(Self { path })
            }
            None => bail!(ErrorKind::UnknownHomeDirectory),
        }
    }

    pub fn default() -> Result<Self> {
        Self::new(DEFAULT_MONGER_DIRECTORY)
    }

    fn create(&self) -> Result<()> {
        create_dir_all(self.path.as_path()).map_err(From::from)
    }

    pub fn write_file(&self, filename: &str, bytes: &[u8]) -> Result<()> {
        self.create()?;

        let filepath = self.path.join(filename);
        let mut file = OpenOptions::new().write(true).create(true).open(filepath)?;
        file.write_all(bytes).map_err(From::from)
    }

    pub fn decompress(&self, filename: &str) -> Result<()> {
        run_command("tar", vec!["xf", filename], self.path.as_path())
    }
}
