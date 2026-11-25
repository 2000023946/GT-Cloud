use crate::domain::{job::Job, resource_states::resource_states::ResourceStates};

pub trait ResourceManager {
    fn configure(&self, job: &Job) -> Result<(), String>;

    fn start(&self, job: &Job, pid: u32) -> Result<(), String>;

    fn monitor(&self, job_id: String) -> Result<ResourceStates, String>;

    fn cleanup(&self, job_id: String) -> Result<(), String>;
}