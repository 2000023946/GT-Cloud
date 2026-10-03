use crate::domain::job::Job;
use crate::ports::resource_manager::ResourceManager;

pub struct ResourceManagerService;

impl ResourceManagerService {
    pub fn new() -> Self {
        Self
    }
}

impl ResourceManager for ResourceManagerService {
    fn allocate(&self, job: Job) -> Result<(), String> {
        println!("ResourceManager: Allocate {}", job.id);
        Ok(())
    }

    fn release(&self, job: Job) -> Result<(), String> {
        println!("ResourceManager: Release {}", job.id);
        Ok(())
    }
}