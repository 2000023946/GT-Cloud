
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

        let pid = match self.get_process_id(&job_id) {
            Ok(pid) => pid,
            Err(error) => {
                self.record_stop_failure(&job_id, &error);
                return Err(error);
            }
        };

        let result = self.os_adapter.kill(pid, libc::SIGTERM);

        self.record_result(result, &job_id, pid)
    }

    fn record_result(
        &self,
        result: libc::c_int,
        job_id: &str,
        pid: libc::pid_t,
    ) -> Result<(), String> {
        if result != -1 {
            self.record_stop_success(job_id, pid);
            return Ok(());
        }

        let error = self.os_adapter.last_os_error();

        let message = match error.raw_os_error() {
            Some(libc::ESRCH) => "Process does not exist".to_string(),
            Some(libc::EPERM) => {
                "Permission denied when stopping process".to_string()
            }
            _ => format!("Failed to stop process: {}", error),
        };

        self.record_stop_failure(job_id, &message);
        Err(message)
    }

    fn get_process_id(
        &self,
        job_id: &str,
    ) -> Result<libc::pid_t, String> {
        let process = self
            .runtime_state
            .get_process(job_id)
            .ok_or_else(|| format!("No process found for job {}", job_id))?;

        let pid: u32 = process
            .pid
            .ok_or_else(|| format!("No PID found for job {}", job_id))?;

        libc::pid_t::try_from(pid)
            .map_err(|_| format!("Invalid PID for job {}", job_id))
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
