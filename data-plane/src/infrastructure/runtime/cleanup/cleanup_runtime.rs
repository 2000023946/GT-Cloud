
use crate::{
    domain::job::Job,
    infrastructure::runtime::{
        cleanup::cleanup_runtime_telemetry::CleanupRuntimeTelemetry,
        states::runtime_state::RuntimeState,
    },
    ports::{logger::Logger, metrics::Metrics},
};

pub struct CleanupRuntime<L: Logger, M: Metrics> {
    pub(crate) telemetry: CleanupRuntimeTelemetry<L, M>,
}

impl<L: Logger, M: Metrics> CleanupRuntime<L, M> {
    pub fn new(telemetry: CleanupRuntimeTelemetry<L, M>) -> Self {
        Self { telemetry }
    }

    pub fn cleanup(&self, state: &RuntimeState, job: Job) -> Result<(), String> {
        let job_id = job.id;

        state.remove_process(&job_id);
        state.remove_stack(&job_id);

        self.telemetry.log_cleanup(&job_id);

        Ok(())
    }
}
