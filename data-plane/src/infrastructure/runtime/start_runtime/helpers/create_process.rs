
#[cfg(target_os = "linux")]
use libc::{
    clone, CLONE_NEWIPC, CLONE_NEWNS, CLONE_NEWPID, CLONE_NEWUSER, CLONE_NEWUTS, SIGCHLD,
};

#[cfg(target_os = "linux")]
use crate::{
    infrastructure::runtime::start_runtime::helpers::child_process::ChildProcess,
    observability::observability::Observability,
    ports::{logger::Logger, metrics::Metrics},
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
    create_process_with(stack, context, |stack_top, flags, context_ptr| unsafe {
        clone(
            ChildProcess::run,
            stack_top,
            flags,
            context_ptr,
        )
    }, || {
        observability.metrics.increment("runtime.start.failure", 1.0);
    })
}

#[cfg(target_os = "linux")]
pub(super) fn create_process_with<T, F, M>(
    stack: &mut Vec<u8>,
    context: T,
    clone_fn: F,
    on_failure: M,
) -> Result<u32, String>
where
    F: FnOnce(
        *mut libc::c_void,
        libc::c_int,
        *mut libc::c_void,
    ) -> libc::pid_t,
    M: FnOnce(),
{
    let flags =
        CLONE_NEWPID | CLONE_NEWNS | CLONE_NEWUTS | CLONE_NEWIPC | CLONE_NEWUSER | SIGCHLD;

    let context_ptr = Box::into_raw(Box::new(context));
    let stack_top = unsafe {
        stack.as_mut_ptr().add(stack.len()) as *mut libc::c_void
    };

    let child_pid = clone_fn(
        stack_top,
        flags,
        context_ptr as *mut libc::c_void,
    );

    if child_pid == -1 {
        unsafe {
            drop(Box::from_raw(context_ptr));
        }

        on_failure();

        return Err(std::io::Error::last_os_error().to_string());
    }

    Ok(child_pid as u32)
}