use crate::domain::{job::Job, resource_state::resource_state::ResourceState};

pub trait ResourceManager {
    fn configure(&self, job: Job) -> Result<(), String>;

    fn monitor(&self, job: Job) -> Result<ResourceState, String>;

    fn cleanup(&self, job: Job) -> Result<(), String>;
}