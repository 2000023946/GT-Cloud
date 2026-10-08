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

#[cfg(target_os = "linux")]
use crate::{
    infrastructure::runtime::start_runtime::helpers::child_process::ChildProcess,
    observability::observability::Observability,
    ports::{
        logger::Logger,
        metrics::Metrics,
    },
};

#[cfg(target_os = "linux")]
pub fn create_process<L, M>(
    observability: &Observability<L, M>,
    stack: &mut Vec<u8>,
    context: ChildProcess,
) -> Result<u32, String>
where
    L: Logger,
    M: Metrics,
{
    let flags =
        CLONE_NEWPID
        | CLONE_NEWNS
        | CLONE_NEWUTS
        | CLONE_NEWIPC
        | CLONE_NEWUSER
        | SIGCHLD;

    let context_ptr =
        Box::into_raw(Box::new(context));

    let child_pid = unsafe {
        clone(
            ChildProcess::run,
            stack
                .as_mut_ptr()
                .add(stack.len())
                as *mut libc::c_void,
            flags,
            context_ptr as *mut libc::c_void,
        )
    };

    if child_pid == -1 {
        unsafe {
            drop(
                Box::from_raw(context_ptr),
            );
        }

        observability
            .metrics
            .increment(
                "runtime.start.failure",
                1.0,
            );

        return Err(
            std::io::Error::last_os_error()
                .to_string(),
        );
    }

    Ok(child_pid as u32)
}