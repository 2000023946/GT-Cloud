
#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, Instant};

    use crate::infrastructure::runtime::stop_runtime::stop_runtime_os_adapter::{
        LinuxStopRuntimeOsAdapter,
        StopRuntimeOsAdapter,
    };

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

    #[test]
    fn test_waitpid_reaps_terminated_child_process() {
        let adapter = LinuxStopRuntimeOsAdapter;

        // Create a child process.
        let child_pid = unsafe { libc::fork() };

        assert!(child_pid >= 0, "Failed to fork child process");

        if child_pid == 0 {
            // Child exits immediately.
            unsafe {
                libc::_exit(0);
            }
        }

        // Parent waits for the child to terminate.
        let mut status: libc::c_int = 0;

        let result = adapter.waitpid(
            child_pid,
            &mut status,
            0,
        );

        assert_eq!(result, child_pid);
        assert!(libc::WIFEXITED(status));
        assert_eq!(libc::WEXITSTATUS(status), 0);
    }

    #[test]
    fn test_sleep_waits_for_requested_duration() {
        let adapter = LinuxStopRuntimeOsAdapter;

        let duration = Duration::from_millis(10);

        let start = Instant::now();

        adapter.sleep(duration);

        let elapsed = start.elapsed();

        assert!(
            elapsed >= duration,
            "Sleep returned too early: {:?}",
            elapsed
        );
    }
}
