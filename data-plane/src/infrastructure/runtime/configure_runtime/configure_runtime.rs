use crate::domain::job::Job;
use crate::infrastructure::runtime::state::{ProcessState, RuntimeState};

pub struct ConfigureRuntime;

impl ConfigureRuntime {
    pub fn configure(
        &self,
        state: &mut RuntimeState,
        job: Job,
    ) -> Result<(), String> {
        let mut command_parts = job.app.command.split_whitespace();

        let program = command_parts
            .next()
            .ok_or_else(|| "Command cannot be empty".to_string())?
            .to_string();

        let args = command_parts
            .map(String::from)
            .collect();

        let process_state = ProcessState {
            program,
            args,
            working_directory: Some(job.app.code_path),
            environment: job.app.environment,
        };

        state.processes.insert(job.id, process_state);

        Ok(())
    }
}