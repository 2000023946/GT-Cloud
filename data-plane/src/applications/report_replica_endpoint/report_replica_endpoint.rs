
use crate::domain::endpoint::Endpoint;
use crate::ports::control_plane_client::ControlPlaneClient;

pub struct ReportReplicaEndpoint<C: ControlPlaneClient> {
    control_plane_client: C,
}

impl<C: ControlPlaneClient> ReportReplicaEndpoint<C> {
    pub fn new(control_plane_client: C) -> Self {
        Self {
            control_plane_client,
        }
    }

    pub fn execute(&self, endpoint: Endpoint) -> Result<(), String> {
        self.control_plane_client.report_endpoint(endpoint)
    }
}