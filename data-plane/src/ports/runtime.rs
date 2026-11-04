use crate::domain::{job::Job, runtime_states::runtime_states::RuntimeStates};

pub trait Runtime {
    fn configure(&self, job: Job) -> Result<(), String>;

    fn start(&self, job: Job) -> Result<(), String>;

    fn monitor(&self, job: Job) -> Result<RuntimeStates, String>;

    fn stop(&self, job: Job) -> Result<(), String>;

    fn cleanup(&self, job: Job) -> Result<(), String>;
}