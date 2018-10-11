use std::ffi::OsStr;
use std::os::unix::process::CommandExt;
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

    ensure!(
        status.success(),
        ErrorKind::FailedSubprocess(cmd.to_string(), status.code())
    );

    Ok(())
}

pub fn exec_command<S>(cmd: &str, args: Vec<S>) -> Result<()>
where
    S: AsRef<OsStr>,
{
    print!("running {}", cmd);

    for arg in &args {
        print!(" {}", arg.as_ref().to_string_lossy());
    }

    println!();

    let mut command = Command::new(cmd);
    command.args(args);

    Err(command.exec().into())
}
