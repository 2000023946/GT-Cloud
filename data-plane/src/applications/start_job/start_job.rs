use crate::domain::job::Job;

pub trait StartJob {
    fn start(&self, job: Job) -> Result<(), String>;
}