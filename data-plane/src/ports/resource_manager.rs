use crate::domain::{job::Job, resource_states::resource_states::ResourceStates};

pub trait ResourceManager {
    fn configure(&self, job: Job) -> Result<(), String>;

    fn monitor(&self, job: Job) -> Result<ResourceStates, String>;

    fn cleanup(&self, job: Job) -> Result<(), String>;
}