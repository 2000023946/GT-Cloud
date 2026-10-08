use crate::{domain::job::Job, infrastructure::runtime::{helpers::working_directory_manager::WorkingDirectoryManager, states::process_state::ProcessState}};
use crate::infrastructure::runtime::states::runtime_state::RuntimeState;
pub struct ConfigureRuntime {
    pub(crate) working_directory_manager: WorkingDirectoryManager,
}

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

        let working_directory = self
            .working_directory_manager
            .get_directory(&job.id);

        let process_state = ProcessState {
            program,
            args,
            working_directory: Some(working_directory),
            environment: job.app.environment,
        };

        state.processes.insert(job.id, process_state);

        Ok(())
    }
}