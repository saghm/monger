use std::ffi::OsStr;
use std::path::Path;
use std::process::Command;

use error::{ErrorKind, Result};

pub fn run_command<I, S, P>(cmd: &str, args: I, dir: P) -> Result<()>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
    P: AsRef<Path>,
{
    let status = Command::new(cmd)
        .current_dir(dir)
        .args(args)
        .spawn()?
        .wait()?;

    println!("status: {}", status);

    ensure!(
        status.success(),
        ErrorKind::FailedSubprocess(cmd.to_string(), status.code())
    );

    Ok(())
}
