
#[cfg(target_os = "linux")]
use std::{
    fs::{File, OpenOptions},
    os::unix::process::CommandExt,
    path::Path,
    process::{Command, Stdio},
};

#[cfg(target_os = "linux")]
pub struct ChildProcess {
    pub program: String,
    pub args: Vec<String>,
    pub working_directory: Option<String>,
    pub environment: Vec<(String, String)>,
}

#[cfg(target_os = "linux")]
impl ChildProcess {
    pub extern "C" fn run(arg: *mut libc::c_void) -> libc::c_int {
        if arg.is_null() {
            eprintln!("ChildProcess received a null context");
            return 127;
        }

        let context = unsafe {
            Box::from_raw(arg as *mut ChildProcess)
        };

        let log_directory = context
            .working_directory
            .as_deref()
            .unwrap_or(".");

        let stdout_path = Path::new(log_directory).join("stdout.txt");
        let stderr_path = Path::new(log_directory).join("stderr.txt");

        // Create fresh log files for this execution.
        let stdout_file = match File::create(&stdout_path) {
            Ok(file) => file,
            Err(error) => {
                eprintln!(
                    "Failed to create stdout log {:?}: {}",
                    stdout_path, error
                );
                return 127;
            }
        };

        let stderr_file = match File::create(&stderr_path) {
            Ok(file) => file,
            Err(error) => {
                eprintln!(
                    "Failed to create stderr log {:?}: {}",
                    stderr_path, error
                );
                return 127;
            }
        };

        let mut command = Command::new(&context.program);

        command
            .args(&context.args)
            .stdin(Stdio::null())
            .stdout(Stdio::from(stdout_file))
            .stderr(Stdio::from(stderr_file));

        if let Some(directory) = &context.working_directory {
            command.current_dir(directory);
        }

        command.env_clear();

        for (key, value) in &context.environment {
            command.env(key, value);
        }

        // Replace the child process with the requested application.
        let error = command.exec();

        // If exec fails, report the reason in the job's stderr log.
        eprintln!(
            "Failed to execute {:?} with args {:?}: {}",
            context.program,
            context.args,
            error
        );

        127
    }
}

