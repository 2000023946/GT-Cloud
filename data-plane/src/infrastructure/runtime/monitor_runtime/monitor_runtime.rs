use crate::{
    domain::{
        job::Job,
        runtime_states::runtime_states::RuntimeStates,
    },
    infrastructure::runtime::states::runtime_state::RuntimeState,
    observability::observability::Observability,
    ports::{logger::Logger, metrics::Metrics},
};
use std::collections::HashMap;

type WaitPidFn = fn(
    libc::pid_t,
    *mut libc::c_int,
    libc::c_int,
) -> libc::pid_t;

fn real_waitpid(
    pid: libc::pid_t,
    status: *mut libc::c_int,
    options: libc::c_int,
) -> libc::pid_t {
    unsafe { libc::waitpid(pid, status, options) }
}

pub struct Monitor<L, M>
where
    L: Logger,
    M: Metrics,
{
    pub(crate) runtime_state: RuntimeState,
    pub(crate) observability: Observability<L, M>,
    waitpid_fn: WaitPidFn,
}

impl<L, M> Monitor<L, M>
where
    L: Logger,
    M: Metrics,
{
    pub fn new(
        runtime_state: RuntimeState,
        observability: Observability<L, M>,
    ) -> Self {
        Self {
            runtime_state,
            observability,
            waitpid_fn: real_waitpid,
        }
    }

    #[cfg(test)]
    pub fn with_waitpid(
        runtime_state: RuntimeState,
        observability: Observability<L, M>,
        waitpid_fn: WaitPidFn,
    ) -> Self {
        Self {
            runtime_state,
            observability,
            waitpid_fn,
        }
    }

    pub fn monitor(
        &self,
        job: Job,
    ) -> Result<RuntimeStates, String> {
        let job_id = job.id;

        self.log_monitoring(&job_id);
        self.record_monitoring_metric();

        if let Some(status) =
            self.get_saved_terminal_status(&job_id)
        {
            return Ok(status);
        }

        let process_state =
            match self.runtime_state.get_process(&job_id) {
                Some(state) => state,
                None => return Ok(RuntimeStates::NotFound),
            };

        let pid: libc::pid_t = match process_state.pid {
            Some(pid) => pid as libc::pid_t,
            None => return Ok(RuntimeStates::NotFound),
        };

        let mut status: libc::c_int = 0;
        let result =
            (self.waitpid_fn)(pid, &mut status, libc::WNOHANG);

        let runtime_states =
            self.get_runtime_states(result, pid, status)?;

        self.save_state(job_id, runtime_states.clone());

        Ok(runtime_states)
    }

    fn log_monitoring(&self, job_id: &str) {
        self.observability.logger.info(
            "Monitoring job",
            HashMap::from([
                ("job_id".to_string(), job_id.to_string()),
            ]),
        );
    }

    fn record_monitoring_metric(&self) {
        self.observability
            .metrics
            .increment("monitor.checks", 1.0);
    }

    fn get_saved_terminal_status(
        &self,
        job_id: &str,
    ) -> Option<RuntimeStates> {
        if let Some(status) = self.runtime_state.get_status(job_id) {
            if matches!(
                status,
                RuntimeStates::Exited { .. }
                    | RuntimeStates::Signaled { .. }
            ) {
                return Some(status);
            }
        }

        None
    }

    fn get_runtime_states(
        &self,
        result: libc::pid_t,
        pid: libc::pid_t,
        status: libc::c_int,
    ) -> Result<RuntimeStates, String> {
        if result == 0 {
            return Ok(RuntimeStates::Running);
        } else if result == pid {
            if libc::WIFEXITED(status) {
                let code = libc::WEXITSTATUS(status);
                return Ok(RuntimeStates::Exited { code });
            } else if libc::WIFSIGNALED(status) {
                let signal = libc::WTERMSIG(status);
                return Ok(RuntimeStates::Signaled { signal });
            }
        } else if result == -1 {
            return Ok(RuntimeStates::Error);
        }

        Err("Unexpected waitpid result or process status".to_string())
    }

    fn save_state(
        &self,
        job_id: String,
        runtime_states: RuntimeStates,
    ) {
        if matches!(
            runtime_states,
            RuntimeStates::Exited { .. }
                | RuntimeStates::Signaled { .. }
        ) {
            self.runtime_state
                .save_status(job_id, runtime_states);
        }
    }
}