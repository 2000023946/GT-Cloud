use crate::domain::job::Job;

pub trait ReportJobStatus {
    fn report(&self, job: Job) -> Result<(), String>;
}