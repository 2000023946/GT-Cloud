use crate::domain::job::Job;

pub trait NetworkManager {
    fn configure(&self, job: Job) -> Result<(), String>;

    fn monitor(&self, job: Job) -> Result<(), String>;

    fn cleanup(&self, job: Job) -> Result<(), String>;
}