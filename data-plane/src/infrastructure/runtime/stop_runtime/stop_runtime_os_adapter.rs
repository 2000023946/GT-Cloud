pub trait StopRuntimeOsAdapter {
    fn kill(&self, pid: libc::pid_t, signal: libc::c_int) -> libc::c_int;

    fn waitpid(
        &self,
        pid: libc::pid_t,
        status: &mut libc::c_int,
        options: libc::c_int,
    ) -> libc::pid_t;

    fn sleep(&self, duration: std::time::Duration);

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
    
    fn waitpid(
        &self,
        pid: libc::pid_t,
        status: &mut libc::c_int,
        options: libc::c_int,
    ) -> libc::pid_t {
        unsafe { libc::waitpid(pid, status, options) }
    }

    fn sleep(&self, duration: std::time::Duration) {
        std::thread::sleep(duration);
    }
}