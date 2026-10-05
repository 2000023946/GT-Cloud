use crate::domain::job::Job;
use crate::ports::runtime::Runtime;

pub struct RuntimeService;

impl RuntimeService {
    pub fn new() -> Self {
        Self
    }
}

impl Runtime for RuntimeService {
    fn start(&self, job: Job) -> Result<(), String> {
        println!("Runtime: StartJob {}", job.id);
        Ok(())
    }

    fn stop(&self, job: Job) -> Result<(), String> {
        println!("Runtime: StopJob {}", job.id);
        Ok(())
    }
    
    fn configure(&self, job: Job) -> Result<(), String> {
        Ok(())
    }
    
    fn monitor(&self, job: Job) -> Result<(), String> {
        Ok(())
    }
    
    fn cleanup(&self, job: Job) -> Result<(), String> {
        todo!()
    }
}