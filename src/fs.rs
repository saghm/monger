use std::env::home_dir;
use std::fs::{create_dir_all, OpenOptions, remove_file};
use std::io::Write;
use std::path::{Path, PathBuf};

use error::{ErrorKind, Result};
use process::run_command;

const DEFAULT_MONGER_DIRECTORY: &str = "code/projects/monger/ds";
const DEFAULT_MONGER_BIN_DIRECTORY: &str = "mongodb-versions";

#[derive(Debug)]
pub struct Fs {
    home_dir: PathBuf,
    bin_dir: PathBuf,
}

impl Fs {
    pub fn new() -> FsBuilder {
        Default::default()
    }

    #[inline]
    fn get_bin_file<P: AsRef<Path>>(&self, filename: P) -> PathBuf {
        self.bin_dir.join(filename)
    }

    fn create(&self) -> Result<()> {
        create_dir_all(self.home_dir.join(self.bin_dir.as_path()).as_path()).map_err(From::from)
    }


    fn decompress(&self, filename: &Path) -> Result<()> {
        run_command(
            "tar",
            vec!["xf".as_ref(), filename.as_os_str()],
            self.home_dir.as_path(),
        )
    }

    fn delete_file(&self, filename: &Path) -> Result<()> {
        let path = self.home_dir.join(filename);

        if !path.exists() {
            return Ok(());
        }

        remove_file(path).map_err(From::from)
    }

    fn write_file(&self, filename: &Path, bytes: &[u8]) -> Result<()> {
        self.create()?;

        let filepath = self.home_dir.join(filename);
        let mut file = OpenOptions::new().write(true).create(true).open(filepath)?;
        file.write_all(bytes).map_err(From::from)
    }

    pub fn write_mongodb_download(&self, filename: &str, bytes: &[u8]) -> Result<()> {
        let bin_file = self.get_bin_file(filename);

        println!("writing {}...", bin_file.display());
        self.write_file(&bin_file, bytes)?;

        println!("decompressing...");
        self.decompress(&bin_file)?;

        println!("cleaning up...");
        self.delete_file(&bin_file)
    }
}

#[derive(Debug, Default)]
pub struct FsBuilder {
    home_dir: Option<String>,
    bin_dir: Option<String>,
}

impl FsBuilder {
    pub fn with_home_dir(&mut self, home_dir: &str) -> &mut Self {
        self.home_dir = Some(home_dir.into());
        self
    }

    pub fn with_bin_dir(&mut self, bin_dir: &str) -> &mut Self {
        self.bin_dir = Some(bin_dir.into());
        self
    }

    pub fn build(self) -> Result<Fs> {
        match home_dir() {
            Some(mut home_dir) => {
                home_dir.push(self.home_dir.unwrap_or(DEFAULT_MONGER_DIRECTORY.into()));

                let bin_dir = Path::new(
                    &self.bin_dir.unwrap_or(DEFAULT_MONGER_BIN_DIRECTORY.into()),
                ).to_path_buf();

                Ok(Fs { home_dir, bin_dir })
            }
            None => bail!(ErrorKind::UnknownHomeDirectory),
        }
    }
}
