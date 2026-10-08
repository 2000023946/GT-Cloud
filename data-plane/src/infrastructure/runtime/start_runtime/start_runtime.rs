
use crate::{
    domain::job::Job,
    infrastructure::runtime::states::runtime_state::RuntimeState,
    observability::observability::Observability,
    ports::{
        logger::Logger,
        metrics::Metrics,
    },
};

#[cfg(target_os = "linux")]
use crate::infrastructure::runtime::start_runtime::helpers::{
    create_child_process::create_child_process,
    create_child_stack::create_child_stack,
    create_process::create_process,
    record_success::record_success,
};

pub struct StartRuntime<L, M>
where
    L: Logger,
    M: Metrics,
{
    pub(crate) observability: Observability<L, M>,
}

impl<L, M> StartRuntime<L, M>
where
    L: Logger,
    M: Metrics,
{
    pub fn start(
        &self,
        state: &RuntimeState,
        job: Job,
    ) -> Result<(), String> {
        #[cfg(target_os = "linux")]
        {
            let job_id = job.id.clone();

            let process_state =
                state.get_process(&job_id).ok_or_else(|| {
                    format!(
                        "No process state found for job {}",
                        job_id
                    )
                })?;

            let context =
                create_child_process(process_state);

            // Store the stack in RuntimeState so its memory
            // remains owned after this method returns.
            let stack = create_child_stack();

            state.add_stack(
                job_id.clone(),
                stack,
            );

            // Use the stored stack while holding the write lock.
            let child_pid = state.with_stack(
                &job_id,
                |stack| {
                    create_process(
                        &self.observability,
                        stack,
                        context,
                    )
                },
            )??;

            state.set_pid(
                &job_id,
                child_pid,
            )?;

            record_success(
                &self.observability,
                &job_id,
                child_pid,
            );

            Ok(())
        }

        #[cfg(not(target_os = "linux"))]
        {
            let _ = state;
            let _ = job;

            self.observability.metrics.increment(
                "runtime.start.failure",
                1.0,
            );

            Err(
                "Runtime process isolation is only supported on Linux"
                    .to_string(),
            )
        }
    }
}
