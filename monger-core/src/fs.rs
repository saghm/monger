use std::{
    collections::{BinaryHeap, HashMap},
    ffi::OsString,
    fs::{create_dir_all, read_dir, remove_dir_all, remove_file, rename, File, OpenOptions},
    io::{Read, Write},
    path::{Path, PathBuf},
    process::Child,
};

use dirs::home_dir;
use semver::Version;

use crate::{
    error::{Error, Result},
    process::{exec_command, run_background_command, run_foreground_command},
    util::{parse_major_minor_version, select_newer_version},
};

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
    pub fn builder() -> FsBuilder {
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

    fn get_version_bin_dir(&self, version: &str) -> Result<PathBuf> {
        let matched_version_str = self.get_newest_matching_version(version)?;
        let matched_version_path = Path::new(&matched_version_str);
        let path = self.get_bin_file_abs(matched_version_path.join("bin"));

        Ok(path)
    }

    #[inline]
    pub fn version_exists(&self, version: &str) -> bool {
        self.get_version_dir(version).is_dir()
    }

    pub fn clear_db_dir(&self, version: &str) -> Result<bool> {
        let db_dir = self.get_db_file_rel(version);
        let found = self.delete_directory(db_dir)?;

        Ok(found)
    }

    fn get_newest_matching_version(&self, version: &str) -> Result<String> {
        if version == "system" {
            return Ok(version.to_string());
        }

        if self.version_exists(version) {
            return Ok(version.to_string());
        }

        let (target_major, target_minor) = match parse_major_minor_version(version) {
            Some(pair) => pair,
            None => {
                return Err(Error::InvalidVersion {
                    version: version.to_string(),
                })
            }
        };

        let mut newest_patch = None;

        for e in read_dir(self.get_bin_dir())? {
            let entry = e?;

            if !entry.file_type()?.is_dir() {
                continue;
            }

            let v = match Version::parse(&entry.file_name().to_string_lossy()) {
                Ok(v) => v,
                Err(_) => continue,
            };

            if v.major == target_major
                && v.minor == target_minor
                && v.build.is_empty()
                && v.pre.is_empty()
            {
                newest_patch = Some(select_newer_version(newest_patch, v));
            }
        }

        let matching_version = match newest_patch {
            Some(version) => format!("{}", version),
            None => {
                return Err(Error::InvalidVersion {
                    version: version.to_string(),
                })
            }
        };

        Ok(matching_version)
    }

    fn create(&self) -> Result<()> {
        create_dir_all(self.get_file(self.bin_dir.as_path()).as_path())?;
        create_dir_all(self.get_file(self.db_dir.as_path()).as_path())?;

        Ok(())
    }

    fn get_default_args_file(&self) -> PathBuf {
        self.home_dir.join("default-args")
    }

    pub(crate) fn clear_default_args(&self) -> Result<bool> {
        let default_args_file = self.get_default_args_file();

        if !default_args_file.exists() {
            return Ok(false);
        }

        remove_file(default_args_file)?;
        Ok(true)
    }

    pub(crate) fn get_default_args(&self) -> Result<Option<String>> {
        let default_args_file = self.get_default_args_file();

        if !default_args_file.is_file() {
            return Ok(None);
        }

        let mut default_args = String::new();
        File::open(default_args_file)?.read_to_string(&mut default_args)?;

        Ok(Some(default_args))
    }

    pub(crate) fn set_default_args(&self, default_args: &str) -> Result<()> {
        self.write_file("default-args", default_args.as_bytes())
    }

    fn decompress_download<P: AsRef<Path>>(
        &self,
        filename: P,
        dirname: P,
        version: P,
    ) -> Result<()> {
        let temp_dir = self.get_bin_dir().join(dirname.as_ref());
        std::fs::create_dir_all(&temp_dir)?;

        run_foreground_command(
            "tar",
            vec![
                "xf".as_ref(),
                filename.as_ref().as_os_str(),
                "-C".as_ref(),
                dirname.as_ref().as_os_str(),
                "--strip-components".as_ref(),
                "1".as_ref(),
            ],
            self.get_bin_dir(),
        )?;

        let old_name = self.get_bin_file_abs(dirname);
        let new_name = self.get_bin_file_abs(version);

        rename(old_name, new_name)?;

        let tarball_file = self.get_bin_dir().join(filename);
        std::fs::remove_file(tarball_file)?;

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
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(filepath)?;
        file.write_all(bytes)?;

        Ok(())
    }

    pub fn create_or_get_db_dir(&self, version: &str) -> Result<PathBuf> {
        let matching_version = self.get_newest_matching_version(version)?;
        let db_dir = self.get_file(self.get_db_file_rel(matching_version));
        create_dir_all(db_dir.as_path())?;
        Ok(db_dir)
    }

    pub fn delete_mongodb_version(&self, version: &str) -> Result<bool> {
        self.delete_directory(self.get_version_dir(version))
    }

    pub fn write_mongodb_download(
        &self,
        filename: &str,
        dirname: &str,
        bytes: &[u8],
        version: &str,
    ) -> Result<()> {
        let bin_file = self.get_bin_file_rel(filename);

        println!("writing {}...", bin_file.display());
        self.write_file(&bin_file, bytes)?;

        println!("decompressing...");
        self.decompress_download(filename, dirname, version)?;

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

    pub fn prune(&self) -> Result<()> {
        let mut versions: HashMap<(u64, u64), _> = HashMap::new();

        for e in read_dir(self.get_bin_dir())? {
            let entry = e?;

            // Skip unless the entry is a directory.
            if !entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                continue;
            }

            let version = match Version::parse(entry.file_name().to_string_lossy().as_ref()) {
                Ok(v) => v,
                Err(_) => continue,
            };

            versions
                .entry((version.major, version.minor))
                .or_insert_with(BinaryHeap::new)
                .push(version);
        }

        for (_, vs) in versions {
            self.prune_versions(vs)?;
        }

        Ok(())
    }

    fn prune_versions(&self, mut versions: BinaryHeap<Version>) -> Result<()> {
        let latest_stable = loop {
            match versions.pop() {
                Some(version) => {
                    if version.build.is_empty() && version.pre.is_empty() {
                        break version;
                    }
                }
                None => return Ok(()),
            }
        };

        for version in versions {
            self.delete_mongodb_version(&format!("{}", version))?;
            println!(
                "Deleted {} (because {} is installed)",
                version, latest_stable
            );
        }

        Ok(())
    }

    pub fn exec_command(&self, binary_name: &str, args: Vec<OsString>, version: &str) -> Error {
        let binary_path = match self.get_version_bin_dir(version) {
            Ok(dir) => dir.join(binary_name),
            Err(e) => return e.into(),
        };

        let dir = match std::env::current_dir() {
            Ok(dir) => dir,
            Err(e) => return e.into(),
        };

        exec_command(binary_path.to_string_lossy().as_ref(), args, dir)
    }

    pub fn run_background_command(
        &self,
        binary_name: &str,
        args: Vec<OsString>,
        version: &str,
    ) -> Result<Child> {
        let binary_path = self.get_version_bin_dir(version)?.join(binary_name);

        run_background_command(
            binary_path.to_string_lossy().as_ref(),
            args,
            std::env::current_dir()?,
        )
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
                home_dir.push(self.home_dir.unwrap_or_else(|| DEFAULT_HOME_DIR.into()));

                let bin_dir = Path::new(&self.bin_dir.unwrap_or_else(|| DEFAULT_BIN_DIR.into()))
                    .to_path_buf();

                let db_dir =
                    Path::new(&self.db_dir.unwrap_or_else(|| DEFAULT_DB_DIR.into())).to_path_buf();

                Ok(Fs {
                    home_dir,
                    bin_dir,
                    db_dir,
                })
            }
            None => Err(Error::UnknownHomeDirectory),
        }
    }
}
