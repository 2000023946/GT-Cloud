use crate::ports::control_plane_client::ControlPlaneClient;

pub struct SendHeartbeatService<C>
where
    C: ControlPlaneClient,
{
    control_plane_client: C,
}

impl<C> SendHeartbeatService<C>
where
    C: ControlPlaneClient,
{
    pub fn new(control_plane_client: C) -> Self {
        Self {
            control_plane_client,
        }
    }

    pub fn send(&self) -> Result<(), String> {
        self.control_plane_client.send_heartbeat()
    }
}