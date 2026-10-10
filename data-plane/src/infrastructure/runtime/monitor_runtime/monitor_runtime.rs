use crate::{domain::{job::Job, runtime_states::runtime_states::RuntimeStates}, infrastructure::runtime::{self, states::runtime_state::RuntimeState}, observability::observability::Observability, ports::{logger::Logger, metrics::Metrics}};

pub struct Monitor<L, M>
where
    L: Logger,
    M: Metrics,
{
    pub(crate) runtime_state: RuntimeState,
    pub(crate) observability: Observability<L, M>,
}

impl<L, M> Monitor<L, M>
where
    L: Logger,
    M: Metrics,
{
    pub fn monitor(
        &self,
        job: Job,
    ) -> Result<RuntimeStates, String> {
        // 1. Get the job ID.
        let job_id = job.id;
        // Return the saved terminal status if one exists.
        if let Some(status) = self.get_saved_terminal_status(&job_id) {
            return Ok(status);
        }
        // 2. Retrieve its ProcessState from RuntimeState.
        let process_state = match self.runtime_state.get_process(&job_id) {
            Some(state) => state,
            None => return Ok(RuntimeStates::NotFound),
        };
        // 3. Get the PID from the process state.
        let pid: i32 = match process_state.pid {
            Some(pid) => pid as i32,
            None => return Ok(RuntimeStates::NotFound),
        };
        // 4. Call waitpid() with WNOHANG.
        let mut status: libc::c_int = 0;
        let result = unsafe {
            libc::waitpid(pid as libc::pid_t, &mut status, libc::WNOHANG)
        };
        // 5. Interpret the process status.
        let runtime_states = self.get_runtime_states(result, pid as libc::pid_t, status)?;
        // 6. Preserve the terminal state. if the process has terminated, save its status.
        self.save_state(job_id, runtime_states.clone());
        // 7. Return RuntimeStates.
        return Ok(runtime_states);
    }

    fn get_saved_terminal_status(&self, job_id: &str) -> Option<RuntimeStates> {
        if let Some(status) = self.runtime_state.get_status(job_id) {
            if matches!(
                status,
                RuntimeStates::Exited { .. } | RuntimeStates::Signaled { .. }
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
        // Interpret the waitpid result and status.
        if result == 0 {
            // process still running
            return Ok(RuntimeStates::Running);
        } else if result == pid {
            // process has terminated
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
        // Unexpected waitpid result or process status
        return Err("Unexpected waitpid result or process status".to_string());
    }

    fn save_state(&self, job_id: String, runtime_states: RuntimeStates) {
        if matches!(
            runtime_states,
            RuntimeStates::Exited { .. } | RuntimeStates::Signaled { .. }
        ) {
            self.runtime_state.save_status(job_id, runtime_states);
        }
    }
}