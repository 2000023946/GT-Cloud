use std::collections::HashMap;

#[cfg(target_os = "linux")]
use std::os::unix::process::CommandExt;

#[cfg(target_os = "linux")]
use libc::{
    clone,
    CLONE_NEWIPC,
    CLONE_NEWNS,
    CLONE_NEWPID,
    CLONE_NEWUSER,
    CLONE_NEWUTS,
    SIGCHLD,
};

use crate::{
    domain::job::Job,
    infrastructure::runtime::states::runtime_state::RuntimeState,
    observability::observability::Observability,
    ports::{
        logger::Logger,
        metrics::Metrics,
    },
};

pub struct StartRuntime<L, M>
where
    L: Logger,
    M: Metrics,
{
    pub(crate) observability: Observability<L, M>,
}

#[cfg(target_os = "linux")]
struct ChildProcess {
    program: String,
    args: Vec<String>,
    working_directory: Option<String>,
    environment: Vec<(String, String)>,
}

#[cfg(target_os = "linux")]
extern "C" fn child_process(
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

    for (key, value) in &context.environment {
        command.env(key, value);
    }

    // exec() replaces this process with the user's program.
    // It only returns when execution fails.
    let error = command.exec();

    eprintln!(
        "Failed to execute {}: {}",
        context.program,
        error
    );

    1
}

impl<L, M> StartRuntime<L, M>
where
    L: Logger,
    M: Metrics,
{
    pub fn start(
        &self,
        state: &RuntimeState,
        job: Job,
    ) -> Result<(), String> {

        #[cfg(target_os = "linux")]
        {
            let job_id = job.id.clone();

            // Find the process configuration created by configure().
            // RuntimeState handles the internal read lock.
            let process_state = state
                .get_process(&job_id)
                .ok_or_else(|| {
                    format!(
                        "No process state found for job {}",
                        job_id
                    )
                })?;

            // Copy everything the child process needs.
            let context = ChildProcess {
                program: process_state.program.clone(),
                args: process_state.args.clone(),
                working_directory: process_state
                    .working_directory
                    .clone(),
                environment: process_state
                    .environment
                    .clone(),
            };

            // clone() requires a separate stack for the child.
            let mut stack = vec![0u8; 1024 * 1024];

            // Create separate Linux namespaces for this job.
            let flags =
                CLONE_NEWPID
                | CLONE_NEWNS
                | CLONE_NEWUTS
                | CLONE_NEWIPC
                | CLONE_NEWUSER
                | SIGCHLD;

            // Transfer ownership of the context to the child.
            let context_ptr =
                Box::into_raw(Box::new(context));

            // Create the isolated child process.
            let child_pid = unsafe {
                clone(
                    child_process,
                    stack
                        .as_mut_ptr()
                        .add(stack.len())
                        as *mut libc::c_void,
                    flags,
                    context_ptr as *mut libc::c_void,
                )
            };

            // clone() failed.
            if child_pid == -1 {
                unsafe {
                    drop(Box::from_raw(context_ptr));
                }

                self.observability
                    .metrics
                    .increment(
                        "runtime.start.failure",
                        1.0,
                    );

                return Err(
                    std::io::Error::last_os_error()
                        .to_string()
                );
            }

            // Save the PID through RuntimeState.
            // RuntimeState handles the internal write lock.
            state.set_pid(
                &job_id,
                child_pid as u32,
            )?;

            // Log successful start.
            let mut fields = HashMap::new();

            fields.insert(
                "job_id".to_string(),
                job_id,
            );

            fields.insert(
                "pid".to_string(),
                child_pid.to_string(),
            );

            self.observability
                .logger
                .info(
                    "Runtime started job",
                    fields,
                );

            // Record successful start.
            self.observability
                .metrics
                .increment(
                    "runtime.start.success",
                    1.0,
                );

            Ok(())
        }

        #[cfg(not(target_os = "linux"))]
        {
            let _ = state;
            let _ = job;

            self.observability
                .metrics
                .increment(
                    "runtime.start.failure",
                    1.0,
                );

            Err(
                "Runtime process isolation is only supported on Linux"
                    .to_string()
            )
        }
    }
}