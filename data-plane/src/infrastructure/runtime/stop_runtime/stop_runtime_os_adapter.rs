pub trait StopRuntimeOsAdapter {
    fn kill(
        &self,
        pid: libc::pid_t,
        signal: libc::c_int,
    ) -> libc::c_int;

    fn last_os_error(&self) -> std::io::Error;
}

pub struct LinuxStopRuntimeOsAdapter;

impl StopRuntimeOsAdapter for LinuxStopRuntimeOsAdapter {
    fn kill(
        &self,
        pid: libc::pid_t,
        signal: libc::c_int,
    ) -> libc::c_int {
        unsafe { libc::kill(pid, signal) }
    }

    fn last_os_error(&self) -> std::io::Error {
        std::io::Error::last_os_error()
    }
}