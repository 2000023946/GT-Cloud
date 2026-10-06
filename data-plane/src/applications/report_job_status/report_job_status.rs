use crate::domain::job::Job;
use crate::ports::control_plane_client::ControlPlaneClient;

pub struct ReportJobStatusService<C>
where
    C: ControlPlaneClient,
{
    control_plane_client: C,
}

impl<C> ReportJobStatusService<C>
where
    C: ControlPlaneClient,
{
    pub fn new(control_plane_client: C) -> Self {
        Self {
            control_plane_client,
        }
    }

    pub fn _report(&self, job: Job) -> Result<(), String> {
        self.control_plane_client.report_job_status(job)
    }
}