
use crate::{
    domain::job::Job,
    infrastructure::runtime::{
        states::runtime_state::RuntimeState,
        stop_runtime::{
            stop_runtime_os_adapter::StopRuntimeOsAdapter,
            stop_runtime_telemetry::StopRuntimeTelemetry,
        },
    },
    ports::{logger::Logger, metrics::Metrics},
};

pub struct StopRuntime<A: StopRuntimeOsAdapter, L: Logger, M: Metrics> {
    pub(crate) runtime_state: RuntimeState,
    pub(crate) telemetry: StopRuntimeTelemetry<L, M>,
    os_adapter: A,
}

impl<A: StopRuntimeOsAdapter, L: Logger, M: Metrics> StopRuntime<A, L, M> {
    pub fn new(
        runtime_state: RuntimeState,
        telemetry: StopRuntimeTelemetry<L, M>,
        os_adapter: A,
    ) -> Self {
        Self {
            runtime_state,
            telemetry,
            os_adapter,
        }
    }

    pub fn stop(&self, job: Job) -> Result<(), String> {
        let job_id = job.id;
        let pid = self.resolve_process_id(&job_id)?;

        // Request graceful termination.
        if self.send_signal(pid, libc::SIGTERM)? {
            self.record_stop_success(&job_id, pid);
            return Ok(());
        }

        // Wait for graceful termination before forcing the process to stop.
        if self.wait_for_process_termination(pid, 2)? {
            self.record_stop_success(&job_id, pid);
            return Ok(());
        }

        // Force termination if the process is still running.
        if self.send_signal(pid, libc::SIGKILL)? {
            self.record_stop_success(&job_id, pid);
            return Ok(());
        }

        // Fail if the process remains alive after SIGKILL.
        self.confirm_process_termination(pid, 2)?;

        self.record_stop_success(&job_id, pid);
        Ok(())
    }

    fn resolve_process_id(&self, job_id: &str) -> Result<libc::pid_t, String> {
        let process = self
            .runtime_state
            .get_process(job_id)
            .ok_or_else(|| format!("No process found for job {}", job_id));

        let pid = match process {
            Ok(process) => process.pid.ok_or_else(|| {
                format!("No PID found for job {}", job_id)
            }),
            Err(error) => Err(error),
        };

        let pid = match pid {
            Ok(pid) => pid,
            Err(error) => {
                self.record_stop_failure(job_id, &error);
                return Err(error);
            }
        };

        match libc::pid_t::try_from(pid) {
            Ok(pid) => Ok(pid),
            Err(_) => {
                let error = format!("Invalid PID for job {}", job_id);
                self.record_stop_failure(job_id, &error);
                Err(error)
            }
        }
    }

    /// Sends a signal to the process.
    ///
    /// Returns:
    /// - Ok(true) if the process no longer exists.
    /// - Ok(false) if the signal was sent successfully.
    /// - Err(error) if signaling failed.
    fn send_signal(
        &self,
        pid: libc::pid_t,
        signal: libc::c_int,
    ) -> Result<bool, String> {
        let result = self.os_adapter.kill(pid, signal);

        if result != -1 {
            return Ok(false);
        }

        let error = self.os_adapter.last_os_error();

        match error.raw_os_error() {
            Some(libc::ESRCH) => Ok(true),

            Some(libc::EPERM) => Err(format!(
                "Permission denied when signaling process {}",
                pid
            )),

            _ => Err(format!(
                "Failed to signal process {}: {}",
                pid, error
            )),
        }
    }

    /// Waits until the process exits or the timeout expires.
    fn wait_for_process_termination(
        &self,
        pid: libc::pid_t,
        timeout_secs: u64,
    ) -> Result<bool, String> {
        let start_time = std::time::Instant::now();
        let timeout = std::time::Duration::from_secs(timeout_secs);

        while start_time.elapsed() < timeout {
            let mut status: libc::c_int = 0;

            let result =
                self.os_adapter
                    .waitpid(pid, &mut status, libc::WNOHANG);

            if result == pid {
                return Ok(true);
            }

            if result == -1 {
                let error = self.os_adapter.last_os_error();

                if error.raw_os_error() == Some(libc::EINTR) {
                    continue;
                }

                return Err(format!(
                    "Failed to wait for process {}: {}",
                    pid, error
                ));
            }

            self.os_adapter
                .sleep(std::time::Duration::from_millis(100));
        }

        Ok(false)
    }

    /// Waits for termination after SIGKILL and reports failure on timeout.
    fn confirm_process_termination(
        &self,
        pid: libc::pid_t,
        timeout_secs: u64,
    ) -> Result<(), String> {
        match self.wait_for_process_termination(pid, timeout_secs) {
            Ok(true) => Ok(()),
            Ok(false) => Err(format!(
                "Failed to terminate process with PID {}",
                pid
            )),
            Err(error) => Err(error),
        }
    }

    fn record_stop_success(&self, job_id: &str, pid: libc::pid_t) {
        self.telemetry.log_stop_success(job_id, pid);
        self.telemetry.record_stop_success();
    }

    fn record_stop_failure(&self, job_id: &str, error: &str) {
        self.telemetry.log_stop_failure(job_id, error);
        self.telemetry.record_stop_failure();
    }
}
