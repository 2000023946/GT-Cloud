
#[cfg(test)]
mod tests {
    use crate::infrastructure::runtime::stop_runtime::stop_runtime_os_adapter::{LinuxStopRuntimeOsAdapter, StopRuntimeOsAdapter};

use super::*;

    #[test]
    fn test_kill_succeeds_for_current_process_with_signal_zero() {
        let adapter = LinuxStopRuntimeOsAdapter;
        let pid = unsafe { libc::getpid() };

        // Signal 0 checks whether the process exists and is accessible.
        // It does not actually send a signal.
        let result = adapter.kill(pid, 0);

        assert_eq!(result, 0);
    }

    #[test]
    fn test_kill_fails_for_nonexistent_process() {
        let adapter = LinuxStopRuntimeOsAdapter;

        // pid_t::MAX is outside the normal valid PID range on Linux/macOS.
        let result = adapter.kill(libc::pid_t::MAX, 0);

        assert_eq!(result, -1);
    }

    #[test]
    fn test_last_os_error_returns_process_not_found_error() {
        let adapter = LinuxStopRuntimeOsAdapter;

        // Cause an OS error immediately before retrieving it.
        let result = adapter.kill(libc::pid_t::MAX, 0);
        let error = adapter.last_os_error();

        assert_eq!(result, -1);
        assert_eq!(error.raw_os_error(), Some(libc::ESRCH));
    }
}
