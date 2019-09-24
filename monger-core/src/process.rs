use std::{
    ffi::OsStr,
    os::unix::process::CommandExt,
    path::Path,
    process::{Command, Stdio},
};

use crate::error::{Error, Result};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ChildType {
    Exec,
    Fork,
    Wait,
}

impl Default for ChildType {
    fn default() -> Self {
        Self::Wait
    }
}

fn exec_command<I, S, P>(cmd: &str, args: I, dir: P) -> Result<()>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
    P: AsRef<Path>,
{
    return Err(Error::Io {
        inner: Command::new(cmd).current_dir(dir).args(args).exec(),
    });
}

pub(crate) fn run_command<I, S, P>(cmd: &str, args: I, dir: P, child_type: ChildType) -> Result<()>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
    P: AsRef<Path>,
{
    if let ChildType::Exec = child_type {
        return exec_command(cmd, args, dir);
    }

    let mut child = Command::new(cmd)
        .current_dir(dir)
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .spawn()?;

    if let ChildType::Fork = child_type {
        return Ok(());
    }

    let status = child.wait()?;

    if !status.success() {
        return Err(Error::FailedSubprocess {
            command: cmd.to_string(),
            exit_code: status.code(),
        });
    }

    Ok(())
}
