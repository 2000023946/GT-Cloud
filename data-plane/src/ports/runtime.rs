use crate::domain::{job::Job, runtime_state::runtime_state::RuntimeState};

pub trait Runtime {
    fn configure(&self, job: Job) -> Result<(), String>;

    fn start(&self, job: Job) -> Result<(), String>;

    fn monitor(&self, job: Job) -> Result<RuntimeState, String>;

    fn stop(&self, job: Job) -> Result<(), String>;

    fn cleanup(&self, job: Job) -> Result<(), String>;
}