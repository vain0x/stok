use std::{io, process::Command};

/// Execute a command. This function doesn't return if success.
///
/// - Call `execcvp` syscall (on Unix), replacing the current process
/// - Spawn the command (on Windows) and wait for it to exit
pub fn exec(cmd: &mut Command) -> io::Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        Err(cmd.exec())
    }
    #[cfg(not(unix))]
    {
        use std::process;
        let status = cmd.spawn()?.wait()?;
        process::exit(status.code().unwrap_or(1));
    }
}
