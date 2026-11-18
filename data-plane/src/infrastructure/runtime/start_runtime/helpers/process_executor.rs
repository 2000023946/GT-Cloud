
#[cfg(target_os = "linux")]
use std::{
    fs::File,
    io,
    os::unix::process::CommandExt,
    process::{Command, Stdio},
};

/// Abstraction for configuring and executing an operating-system process.
pub trait ProcessExecutor {
    fn execute(
        &self,
        program: &str,
        args: &[String],
        working_directory: Option<&str>,
        environment: &[(String, String)],
        stdout: File,
        stderr: File,
    ) -> io::Result<()>;
}

/// Production Linux process executor.
///
/// Uses exec() to replace the current process with the requested program.
/// Returns an error only when process execution fails.
#[cfg(target_os = "linux")]
pub struct LinuxProcessExecutor;

#[cfg(target_os = "linux")]
impl ProcessExecutor for LinuxProcessExecutor {
    fn execute(
        &self,
        program: &str,
        args: &[String],
        working_directory: Option<&str>,
        environment: &[(String, String)],
        stdout: File,
        stderr: File,
    ) -> io::Result<()> {
        let mut command = Command::new(program);

        command
            .args(args)
            .stdin(Stdio::null())
            .stdout(Stdio::from(stdout))
            .stderr(Stdio::from(stderr));

        if let Some(directory) = working_directory {
            command.current_dir(directory);
        }

        // Do not inherit the parent process environment.
        command.env_clear();

        for (key, value) in environment {
            command.env(key, value);
        }

        // Successful exec replaces this process.
        // If exec fails, it returns an io::Error.
        Err(command.exec())
    }
}
