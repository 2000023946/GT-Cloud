use crate::domain::job::Job;
use crate::ports::resource_manager::ResourceManager;

pub struct ResourceManagerService;

impl ResourceManagerService {
    pub fn new() -> Self {
        Self
    }
}

impl ResourceManager for ResourceManagerService {
    fn configure(&self, job: Job) -> Result<(), String> {
        todo!()
    }
    
    fn monitor(&self, job: Job) -> Result<(), String> {
        todo!()
    }
    
    fn cleanup(&self, job: Job) -> Result<(), String> {
        todo!()
    }
}