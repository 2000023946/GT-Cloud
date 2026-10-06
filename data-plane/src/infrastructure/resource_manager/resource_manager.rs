use crate::domain::job::Job;
use crate::ports::resource_manager::ResourceManager;

pub struct ResourceManagerService;

impl ResourceManagerService {
    pub fn new() -> Self {
        Self
    }
}

impl ResourceManager for ResourceManagerService {
    fn configure(&self, _job: Job) -> Result<(), String> {
        todo!()
    }
    
    fn monitor(&self, _job: Job) -> Result<(), String> {
        todo!()
    }
    
    fn cleanup(&self, _job: Job) -> Result<(), String> {
        todo!()
    }
}