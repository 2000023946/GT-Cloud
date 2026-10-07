use crate::domain::job::Job;

pub trait StopJob {
    fn stop(&self, job: Job) -> Result<(), String>;
}