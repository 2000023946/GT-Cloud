#[cfg(target_os = "linux")]
use std::os::unix::process::CommandExt;
#[cfg(target_os = "linux")]
pub struct ChildProcess {
    pub program: String,
    pub args: Vec<String>,
    pub working_directory: Option<String>,
    pub environment: Vec<(String, String)>,
}

#[cfg(target_os = "linux")]
impl ChildProcess {
    pub extern "C" fn run(
        arg: *mut libc::c_void,
    ) -> libc::c_int {
        let context = unsafe {
            Box::from_raw(arg as *mut ChildProcess)
        };

        let mut command =
            std::process::Command::new(&context.program);

        command.args(&context.args);

        if let Some(directory) = &context.working_directory {
            command.current_dir(directory);
        }

        command.env_clear();

        // Keep the environment supplied by the runtime.
        for (key, value) in &context.environment {
            command.env(key, value);
        }

        // exec replaces this child with the requested program.
        let error = command.exec();

        eprintln!(
            "Failed to execute program {:?}, args {:?}: {}",
            context.program,
            context.args,
            error
        );

        1
    }
}