
use std::collections::HashMap;
use std::sync::RwLock;

use crate::infrastructure::runtime::states::process_state::ProcessState;

pub struct RuntimeState {
    processes: RwLock<HashMap<String, ProcessState>>,
    stacks: RwLock<HashMap<String, Vec<u8>>>,
}

impl RuntimeState {
    pub fn new() -> Self {
        Self {
            processes: RwLock::new(HashMap::new()),
            stacks: RwLock::new(HashMap::new()),
        }
    }

    // Process state methods

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

        let process = processes.get_mut(job_id).ok_or_else(|| {
            format!("No process state found for job {}", job_id)
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

    // Child stack methods

    pub fn add_stack(
        &self,
        job_id: String,
        stack: Vec<u8>,
    ) {
        let mut stacks = self.stacks.write().unwrap();
        stacks.insert(job_id, stack);
    }

    pub fn with_stack<F, T>(
        &self,
        job_id: &str,
        f: F,
    ) -> Result<T, String>
    where
        F: FnOnce(&mut Vec<u8>) -> T,
    {
        let mut stacks = self.stacks.write().unwrap();

        let stack = stacks.get_mut(job_id).ok_or_else(|| {
            format!("No stack found for job {}", job_id)
        })?;

        Ok(f(stack))
    }

    pub fn remove_stack(
        &self,
        job_id: &str,
    ) -> Option<Vec<u8>> {
        let mut stacks = self.stacks.write().unwrap();
        stacks.remove(job_id)
    }

    pub fn contains_stack(
        &self,
        job_id: &str,
    ) -> bool {
        let stacks = self.stacks.read().unwrap();
        stacks.contains_key(job_id)
    }

    pub fn stack_count(&self) -> usize {
        let stacks = self.stacks.read().unwrap();
        stacks.len()
    }
}

