use std::env::home_dir;
use std::fs::{create_dir_all, OpenOptions, read_dir, remove_dir_all, remove_file, rename};
use std::io::Write;
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

use error::{ErrorKind, Result};
use process::{exec_command, run_command};

const DEFAULT_HOME_DIR: &str = ".monger";
const DEFAULT_BIN_DIR: &str = "mongodb-versions";
const DEFAULT_DB_DIR: &str = "db";

#[derive(Debug)]
pub struct Fs {
    home_dir: PathBuf,
    bin_dir: PathBuf,
    db_dir: PathBuf,
}

impl Fs {
    pub fn new() -> FsBuilder {
        Default::default()
    }

    #[inline]
    fn get_file<P: AsRef<Path>>(&self, filename: P) -> PathBuf {
        self.home_dir.join(filename)
    }

    #[inline]
    fn get_bin_file_rel<P: AsRef<Path>>(&self, filename: P) -> PathBuf {
        self.bin_dir.join(filename)
    }

    #[inline]
    fn get_bin_file_abs<P: AsRef<Path>>(&self, filename: P) -> PathBuf {
        self.get_file(self.get_bin_file_rel(filename))
    }

    #[inline]
    fn get_bin_dir(&self) -> PathBuf {
        self.get_file(self.bin_dir.as_path())
    }

    #[inline]
    fn get_db_file_rel<P: AsRef<Path>>(&self, filename: P) -> PathBuf {
        self.db_dir.join(filename)
    }


    #[inline]
    fn get_version_dir(&self, version: &str) -> PathBuf {
        self.get_bin_file_abs(version)
    }

    #[inline]
    fn get_version_bin_dir<P: AsRef<Path>>(&self, version: P) -> PathBuf {
        self.get_bin_file_abs(version.as_ref().join("bin"))
    }


    fn create(&self) -> Result<()> {
        create_dir_all(self.get_file(self.bin_dir.as_path()).as_path())?;
        create_dir_all(self.get_file(self.db_dir.as_path()).as_path())?;

        Ok(())
    }


    fn decompress_download<P: AsRef<Path>>(&self, filename: P, version: P) -> Result<()> {
        run_command(
            "tar",
            vec!["xf".as_ref(), filename.as_ref().as_os_str()],
            self.get_bin_dir(),
        )?;

        // TODO: Deal with unwrap in a non-messy way.
        let old_name = self.get_bin_file_abs(filename.as_ref().file_stem().unwrap());
        let new_name = self.get_bin_file_abs(version);
        rename(old_name, new_name)?;

        Ok(())
    }

    fn delete_file<P: AsRef<Path>>(&self, filename: P) -> Result<()> {
        let path = self.home_dir.join(filename);

        if !path.exists() {
            return Ok(());
        }

        remove_file(path)?;

        Ok(())
    }

    fn delete_directory<P: AsRef<Path>>(&self, dirname: P) -> Result<bool> {
        let path = self.home_dir.join(dirname);

        if !path.exists() {
            return Ok(false);
        }

        remove_dir_all(path)?;

        Ok(true)
    }

    fn write_file<P: AsRef<Path>>(&self, filename: P, bytes: &[u8]) -> Result<()> {
        self.create()?;

        let filepath = self.home_dir.join(filename);
        let mut file = OpenOptions::new().write(true).create(true).open(filepath)?;
        file.write_all(bytes)?;

        Ok(())
    }

    #[inline]
    pub fn version_exists(&self, version: &str) -> bool {
        self.get_version_dir(version).is_dir()
    }

    pub fn create_or_get_db_dir(&self, version: &str) -> Result<PathBuf> {
        let db_dir = self.get_file(self.get_db_file_rel(version));
        create_dir_all(db_dir.as_path())?;
        Ok(db_dir)
    }

    pub fn delete_mongodb_version(&self, version: &str) -> Result<bool> {
        self.delete_directory(self.get_version_dir(version))
    }

    pub fn write_mongodb_download(
        &self,
        filename: &str,
        bytes: &[u8],
        version: &str,
    ) -> Result<()> {
        let bin_file = self.get_bin_file_rel(filename);

        println!("writing {}...", bin_file.display());
        self.write_file(&bin_file, bytes)?;

        println!("decompressing...");
        self.decompress_download(filename, version)?;

        println!("cleaning up...");
        self.delete_file(&bin_file)?;

        Ok(())
    }

    pub fn list_versions(&self) -> Result<Vec<OsString>> {
        self.create()?;

        let mut versions = Vec::new();

        for e in read_dir(self.get_bin_dir())? {
            let entry = e?;

            if entry.file_type()?.is_dir() {
                versions.push(entry.file_name());
            }
        }

        Ok(versions)
    }

    pub fn exec<I, S>(&self, binary_name: &str, args: I, version: &str) -> Result<()>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        exec_command(&format!("./{}", binary_name), args, self.get_version_bin_dir(version))
    }
}

#[derive(Debug, Default)]
pub struct FsBuilder {
    home_dir: Option<String>,
    bin_dir: Option<String>,
    db_dir: Option<String>,
}

impl FsBuilder {
    #[allow(dead_code)]
    pub fn with_home_dir(&mut self, home_dir: &str) -> &mut Self {
        self.home_dir = Some(home_dir.into());
        self
    }

    #[allow(dead_code)]
    pub fn with_bin_dir(&mut self, bin_dir: &str) -> &mut Self {
        self.bin_dir = Some(bin_dir.into());
        self
    }

    #[allow(dead_code)]
    pub fn with_db_dir(&mut self, db_dir: &str) -> &mut Self {
        self.db_dir = Some(db_dir.into());
        self
    }

    pub fn build(self) -> Result<Fs> {
        match home_dir() {
            Some(mut home_dir) => {
                home_dir.push(self.home_dir.unwrap_or(DEFAULT_HOME_DIR.into()));

                let bin_dir = Path::new(&self.bin_dir.unwrap_or(DEFAULT_BIN_DIR.into()))
                    .to_path_buf();

                let db_dir = Path::new(&self.db_dir.unwrap_or(DEFAULT_DB_DIR.into())).to_path_buf();

                Ok(Fs {
                    home_dir,
                    bin_dir,
                    db_dir,
                })
            }
            None => bail!(ErrorKind::UnknownHomeDirectory),
        }
    }
}
