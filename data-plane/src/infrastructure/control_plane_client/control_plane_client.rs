use crate::domain::job::Job;
use crate::ports::control_plane_client::ControlPlaneClient;

pub struct ControlPlaneClientService;

impl ControlPlaneClientService {
    pub fn new() -> Self {
        Self
    }
}

impl ControlPlaneClient for ControlPlaneClientService {
    fn send_heartbeat(&self) -> Result<(), String> {
        println!("ControlPlaneClient: SendHeartbeat");
        Ok(())
    }

    fn report_job_status(&self, job: Job) -> Result<(), String> {
        println!("ControlPlaneClient: ReportJobStatus {}", job.id);
        Ok(())
    }
    
    fn report_endpoint(&self, _endpoint: crate::domain::endpoint::Endpoint) -> Result<(), String> {
        Ok(())
    }
}