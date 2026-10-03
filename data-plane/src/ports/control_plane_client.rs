use crate::domain::job::Job;

pub trait ControlPlaneClient {
    fn send_heartbeat(&self) -> Result<(), String>;
    fn report_job_status(&self, job: Job) -> Result<(), String>;
}