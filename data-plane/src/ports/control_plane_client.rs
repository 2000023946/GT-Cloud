use crate::domain::job::Job;
use crate::domain::endpoint::Endpoint;
pub trait ControlPlaneClient {
    fn send_heartbeat(&self) -> Result<(), String>;
    fn report_job_status(&self, job: Job) -> Result<(), String>;
    fn report_endpoint(&self, endpoint: Endpoint) -> Result<(), String>;
}