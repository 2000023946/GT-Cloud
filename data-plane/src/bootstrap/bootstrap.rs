use crate::api::api::API;

use crate::applications::report_job_status::ReportJobStatusService;
use crate::applications::send_heartbeat::SendHeartbeatService;
use crate::applications::start_job::StartJobService;
use crate::applications::stop_job::StopJobService;

use crate::infrastructure::control_plane_client::control_plane_client::ControlPlaneClientService;
use crate::infrastructure::network_manager::network_manager::NetworkManagerService;
use crate::infrastructure::resource_manager::resource_manager::ResourceManagerService;
use crate::infrastructure::runtime::runtime::RuntimeService;

pub fn start() -> API<
    RuntimeService,
    ResourceManagerService,
    NetworkManagerService,
    ControlPlaneClientService,
> {
    // Infrastructure
    let runtime = RuntimeService::new();
    let resource_manager = ResourceManagerService::new();
    let network_manager = NetworkManagerService::new();
    let control_plane_client = ControlPlaneClientService::new();

    // Applications
    let start_job = StartJobService::new(
        runtime,
        resource_manager,
        network_manager,
    );

    // We need separate infrastructure instances here because
    // the services currently own their dependencies.
    let stop_job = StopJobService::new(
        RuntimeService::new(),
        ResourceManagerService::new(),
        NetworkManagerService::new(),
    );

    let send_heartbeat =
        SendHeartbeatService::new(ControlPlaneClientService::new());

    let report_job_status =
        ReportJobStatusService::new(ControlPlaneClientService::new());

    // API
    API::new(
        start_job,
        stop_job,
        send_heartbeat,
        report_job_status,
    )
}