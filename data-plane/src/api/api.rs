use crate::applications::report_job_status::ReportJobStatusService;
use crate::applications::send_heartbeat::SendHeartbeatService;
use crate::applications::start_job::StartJobService;
use crate::applications::stop_job::StopJobService;
use crate::domain::job::Job;

pub struct API<R, RM, N, C>
where
    R: crate::ports::runtime::Runtime,
    RM: crate::ports::resource_manager::ResourceManager,
    N: crate::ports::network_manager::NetworkManager,
    C: crate::ports::control_plane_client::ControlPlaneClient,
{
    start_job: StartJobService<R, RM, N>,
    stop_job: StopJobService<R, RM, N>,
    send_heartbeat: SendHeartbeatService<C>,
    report_job_status: ReportJobStatusService<C>,
}

impl<R, RM, N, C> API<R, RM, N, C>
where
    R: crate::ports::runtime::Runtime,
    RM: crate::ports::resource_manager::ResourceManager,
    N: crate::ports::network_manager::NetworkManager,
    C: crate::ports::control_plane_client::ControlPlaneClient,
{
    pub fn new(
        start_job: StartJobService<R, RM, N>,
        stop_job: StopJobService<R, RM, N>,
        send_heartbeat: SendHeartbeatService<C>,
        report_job_status: ReportJobStatusService<C>,
    ) -> Self {
        Self {
            start_job,
            stop_job,
            send_heartbeat,
            report_job_status,
        }
    }

    pub fn start_job(&self, job: Job) -> Result<(), String> {
        println!("API: StartJob");
        self.start_job.start(job)
    }

    pub fn stop_job(&self, job: Job) -> Result<(), String> {
        println!("API: StopJob");
        self.stop_job.stop(job)
    }

    pub fn send_heartbeat(&self) -> Result<(), String> {
        println!("API: SendHeartbeat");
        self.send_heartbeat.send()
    }

    pub fn report_job_status(&self, job: Job) -> Result<(), String> {
        println!("API: ReportJobStatus");
        self.report_job_status.report(job)
    }
}