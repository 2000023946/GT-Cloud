use crate::domain::job::Job;

pub trait ResourceManager {
    fn configure(&self, job: Job) -> Result<(), String>;

    fn monitor(&self, job: Job) -> Result<(), String>;

    fn cleanup(&self, job: Job) -> Result<(), String>;
}// this is resource