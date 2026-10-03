use crate::domain::job::Job;
use crate::ports::network_manager::NetworkManager;
use crate::ports::resource_manager::ResourceManager;
use crate::ports::runtime::Runtime;

pub struct StopJobService<R, RM, N>
where
    R: Runtime,
    RM: ResourceManager,
    N: NetworkManager,
{
    runtime: R,
    resource_manager: RM,
    network_manager: N,
}

impl<R, RM, N> StopJobService<R, RM, N>
where
    R: Runtime,
    RM: ResourceManager,
    N: NetworkManager,
{
    pub fn new(
        runtime: R,
        resource_manager: RM,
        network_manager: N,
    ) -> Self {
        Self {
            runtime,
            resource_manager,
            network_manager,
        }
    }

    pub fn stop(&self, job: Job) -> Result<(), String> {
        self.runtime.stop(job.clone())?;
        self.network_manager.cleanup(job.clone())?;
        self.resource_manager.release(job)?;

        Ok(())
    }
}