use std::{
    ffi::{OsStr, OsString},
    os::unix::process::CommandExt,
    path::Path,
    process::{Child, Command, Stdio},
};

use crate::error::{Error, Result};

pub(crate) fn exec_command(cmd: &str, args: Vec<OsString>, dir: impl AsRef<Path>) -> Error {
    Error::Io {
        inner: Command::new(cmd).current_dir(dir).args(args).exec(),
    }
}

pub(crate) fn run_foreground_command(
    cmd: &str,
    args: Vec<impl AsRef<OsStr>>,
    dir: impl AsRef<Path>,
) -> Result<()> {
    let status = run_background_command(cmd, args, dir)?.wait()?;

    if !status.success() {
        return Err(Error::FailedSubprocess {
            command: cmd.to_string(),
            exit_code: status.code(),
        });
    }

    Ok(())
}

pub(crate) fn run_background_command(
    cmd: &str,
    args: Vec<impl AsRef<OsStr>>,
    dir: impl AsRef<Path>,
) -> Result<Child> {
    let child = Command::new(cmd)
        .current_dir(dir)
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .spawn()?;

    Ok(child)
}
