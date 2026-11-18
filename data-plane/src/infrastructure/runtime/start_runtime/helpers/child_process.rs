
#[cfg(target_os = "linux")]
use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::Path,
};

#[cfg(target_os = "linux")]
use super::process_executor::{
    LinuxProcessExecutor,
    ProcessExecutor,
};

#[cfg(target_os = "linux")]
pub struct ChildProcess {
    pub program: String,
    pub args: Vec<String>,
    pub working_directory: Option<String>,
    pub environment: Vec<(String, String)>,
}

#[cfg(target_os = "linux")]
pub fn create_log_file(path: &Path) -> std::io::Result<File> {
    File::create(path)
}

#[cfg(target_os = "linux")]
pub fn append_exec_error(
    stderr_path: &Path,
    program: &str,
    args: &[String],
    error: &std::io::Error,
) {
    let message = format!(
        "Failed to execute {:?} with args {:?}: {}",
        program, args, error
    );

    match OpenOptions::new().append(true).open(stderr_path) {
        Ok(mut file) => {
            let _ = writeln!(file, "{message}");
        }
        Err(log_error) => {
            eprintln!("{message}");
            eprintln!(
                "Failed to append to stderr log {:?}: {}",
                stderr_path, log_error
            );
        }
    }
}

#[cfg(target_os = "linux")]
impl ChildProcess {
    /// C entry point used by the runtime.
    pub extern "C" fn run(arg: *mut libc::c_void) -> libc::c_int {
        if arg.is_null() {
            eprintln!("ChildProcess received a null context");
            return 127;
        }

        let context = unsafe {
            Box::from_raw(arg as *mut ChildProcess)
        };

        Self::run_with_executor(context, &LinuxProcessExecutor)
    }

    /// Internal execution path with an injectable process executor.
    ///
    /// Tests can supply a mock executor without replacing the test process.
    #[cfg(target_os = "linux")]
    pub(crate) fn run_with_executor(
        context: Box<ChildProcess>,
        executor: &dyn ProcessExecutor,
    ) -> libc::c_int {
        let log_directory = context
            .working_directory
            .as_deref()
            .unwrap_or(".");

        let stdout_path = Path::new(log_directory).join("stdout.txt");
        let stderr_path = Path::new(log_directory).join("stderr.txt");

        let stdout_file = match create_log_file(&stdout_path) {
            Ok(file) => file,
            Err(error) => {
                eprintln!(
                    "Failed to create stdout log {:?}: {}",
                    stdout_path, error
                );
                return 127;
            }
        };

        let stderr_file = match create_log_file(&stderr_path) {
            Ok(file) => file,
            Err(error) => {
                eprintln!(
                    "Failed to create stderr log {:?}: {}",
                    stderr_path, error
                );
                return 127;
            }
        };

        match executor.execute(
            &context.program,
            &context.args,
            context.working_directory.as_deref(),
            &context.environment,
            stdout_file,
            stderr_file,
        ) {
            Ok(()) => 0,
            Err(error) => {
                append_exec_error(
                    &stderr_path,
                    &context.program,
                    &context.args,
                    &error,
                );
                127
            }
        }
    }
}
