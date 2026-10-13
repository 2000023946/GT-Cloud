#[cfg(target_os = "linux")]
use crate::{
    infrastructure::runtime::{
        states::process_state::ProcessState,
        start_runtime::helpers::child_process::ChildProcess,
    },
};

#[cfg(target_os = "linux")]
pub fn create_child_process(
    process_state: ProcessState,
) -> ChildProcess {
    ChildProcess {
        program: process_state.program,
        args: process_state.args,
        working_directory: process_state.working_directory,
        environment: process_state.environment,
    }
}