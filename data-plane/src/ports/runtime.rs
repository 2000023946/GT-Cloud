use crate::domain::job::Job;

pub trait Runtime {
    fn configure(&self, job: Job) -> Result<(), String>;

    fn start(&self, job: Job) -> Result<(), String>;

    fn monitor(&self, job: Job) -> Result<(), String>;

    fn stop(&self, job: Job) -> Result<(), String>;

    fn cleanup(&self, job: Job) -> Result<(), String>;
}