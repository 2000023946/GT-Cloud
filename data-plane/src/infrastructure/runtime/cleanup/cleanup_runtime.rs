use crate::{
    domain::job::Job, infrastructure::runtime::cleanup::{cleanup_runtime_state::CleanupRuntimeState, cleanup_runtime_telemetry::CleanupRuntimeTelemetry}, ports::{logger::Logger, metrics::Metrics},
};

pub struct CleanupRuntime<S, L: Logger, M: Metrics>
where
    S: CleanupRuntimeState,
{
    pub(crate) runtime_state: S,
    pub(crate) telemetry: CleanupRuntimeTelemetry<L, M>,
}

impl<S, L: Logger, M: Metrics> CleanupRuntime<S, L, M>
where
    S: CleanupRuntimeState,
{
    pub fn new(
        runtime_state: S,
        telemetry: CleanupRuntimeTelemetry<L, M>,
    ) -> Self {
        Self {
            runtime_state,
            telemetry,
        }
    }

    pub fn cleanup(&self, job: Job) -> Result<(), String> {
        let job_id = job.id;

        let result = (|| -> Result<(), String> {
            // Remove process state
            self.runtime_state.remove_process(&job_id)?;

            // Remove child stack
            self.runtime_state.remove_stack(&job_id)?;

            Ok(())
        })();

        match result {
            Ok(()) => {
                self.telemetry.log_cleanup(&job_id);
                Ok(())
            }
            Err(error) => {
                self.telemetry.log_failure(&job_id, &error);
                Err(error)
            }
        }
    }
}