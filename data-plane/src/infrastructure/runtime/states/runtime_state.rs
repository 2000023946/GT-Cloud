use std::collections::HashMap;
use std::sync::RwLock;

use crate::infrastructure::runtime::states::process_state::ProcessState;

pub struct RuntimeState {
    processes: RwLock<HashMap<String, ProcessState>>,
}

impl RuntimeState {
    pub fn new() -> Self {
        Self {
            processes: RwLock::new(HashMap::new()),
        }
    }

    pub fn get_process(
        &self,
        job_id: &str,
    ) -> Option<ProcessState> {
        let processes = self.processes.read().unwrap();

        processes.get(job_id).cloned()
    }

    pub fn add_process(
        &self,
        job_id: String,
        process: ProcessState,
    ) {
        let mut processes = self.processes.write().unwrap();

        processes.insert(job_id, process);
    }

    pub fn set_pid(
        &self,
        job_id: &str,
        pid: u32,
    ) -> Result<(), String> {
        let mut processes = self.processes.write().unwrap();

        let process = processes
            .get_mut(job_id)
            .ok_or_else(|| {
                format!(
                    "No process state found for job {}",
                    job_id
                )
            })?;

        process.pid = Some(pid);

        Ok(())
    }

    pub fn remove_process(
        &self,
        job_id: &str,
    ) -> Option<ProcessState> {
        let mut processes = self.processes.write().unwrap();

        processes.remove(job_id)
    }

    pub fn contains_process(
        &self,
        job_id: &str,
    ) -> bool {
        let processes = self.processes.read().unwrap();

        processes.contains_key(job_id)
    }

    pub fn process_count(&self) -> usize {
        let processes = self.processes.read().unwrap();

        processes.len()
    }
}