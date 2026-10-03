use crate::domain::job::Job;

pub trait Runtime {
    fn start(&self, job: Job) -> Result<(), String>;
    fn stop(&self, job: Job) -> Result<(), String>;
}