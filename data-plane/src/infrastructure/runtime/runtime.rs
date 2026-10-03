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
}