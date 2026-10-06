use crate::domain::job::Job;
use crate::ports::network_manager::NetworkManager;

pub struct NetworkManagerService;

impl NetworkManagerService {
    pub fn new() -> Self {
        Self
    }
}

impl NetworkManager for NetworkManagerService {
    fn configure(&self, job: Job) -> Result<(), String> {
        println!("NetworkManager: Configure {}", job.id);
        Ok(())
    }

    fn cleanup(&self, job: Job) -> Result<(), String> {
        println!("NetworkManager: Cleanup {}", job.id);
        Ok(())
    }
    
    fn monitor(&self, _job: Job) -> Result<(), String> {
        Ok(())
    }
}