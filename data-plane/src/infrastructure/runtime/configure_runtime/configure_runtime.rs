use std::collections::HashMap;

use crate::{
    domain::job::Job,
    observability::observability::Observability,
    infrastructure::runtime::{
        helpers::working_directory_manager::WorkingDirectoryManager,
        states::{
            process_state::ProcessState,
            runtime_state::RuntimeState,
        },
    },
    ports::{
        logger::Logger,
        metrics::Metrics,
    },
};

pub struct ConfigureRuntime<L, M>
where
    L: Logger,
    M: Metrics,
{
    pub(crate) working_directory_manager: WorkingDirectoryManager,
    pub(crate) observability: Observability<L, M>,
}

impl<L, M> ConfigureRuntime<L, M>
where
    L: Logger,
    M: Metrics,
{
    pub fn configure(
        &self,
        state: &mut RuntimeState,
        job: Job,
    ) -> Result<(), String> {
        let job_id = job.id.clone();

        let mut command_parts = job.app.command.split_whitespace();

        let program = match command_parts.next() {
            Some(program) => program.to_string(),
            None => {
                let mut fields = HashMap::new();
                fields.insert("job_id".to_string(), job_id);

                self.observability
                    .logger
                    .error("Failed to configure runtime: empty command", fields);

                self.observability
                    .metrics
                    .increment("runtime.configure.failure", 1.0);

                return Err("Command cannot be empty".to_string());
            }
        };

        let args = command_parts.map(String::from).collect();

        let working_directory = self
            .working_directory_manager
            .get_directory(&job_id);

        let process_state = ProcessState {
            program,
            args,
            working_directory: Some(working_directory),
            environment: job.app.environment,
        };

        state.processes.insert(job_id.clone(), process_state);

        let mut fields = HashMap::new();
        fields.insert("job_id".to_string(), job_id);

        self.observability
            .logger
            .info("Runtime configured job", fields);

        self.observability
            .metrics
            .increment("runtime.configure.success", 1.0);

        Ok(())
    }
}