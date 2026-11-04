use std::sync::Arc;

use crate::infrastructure::runtime::states::{
    process_state::ProcessState,
    runtime_state::RuntimeState,
};

pub trait CleanupRuntimeState: Send + Sync {
    fn remove_process(
        &self,
        job_id: &str,
    ) -> Result<Option<ProcessState>, String>;

    fn remove_stack(
        &self,
        job_id: &str,
    ) -> Result<Option<Vec<u8>>, String>;
}

// Real RuntimeState implementation
impl CleanupRuntimeState for RuntimeState {
    fn remove_process(
        &self,
        job_id: &str,
    ) -> Result<Option<ProcessState>, String> {
        Ok(RuntimeState::remove_process(self, job_id))
    }

    fn remove_stack(
        &self,
        job_id: &str,
    ) -> Result<Option<Vec<u8>>, String> {
        Ok(RuntimeState::remove_stack(self, job_id))
    }
}

// Arc forwarding implementation
impl<T: CleanupRuntimeState> CleanupRuntimeState for Arc<T> {
    fn remove_process(
        &self,
        job_id: &str,
    ) -> Result<Option<ProcessState>, String> {
        (**self).remove_process(job_id)
    }

    fn remove_stack(
        &self,
        job_id: &str,
    ) -> Result<Option<Vec<u8>>, String> {
        (**self).remove_stack(job_id)
    }
}