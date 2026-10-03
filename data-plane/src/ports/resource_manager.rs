use crate::domain::job::Job;

pub trait ResourceManager {
    fn allocate(&self, job: Job) -> Result<(), String>;
    fn release(&self, job: Job) -> Result<(), String>;
}