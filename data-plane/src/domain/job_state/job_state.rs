use crate::domain::{network_state::network_state::NetworkState, resource_state::resource_state::ResourceState, runtime_state::runtime_state::RuntimeState};


#[derive(Clone)]
pub struct JobState {
    pub job_id: String,
    pub runtime: RuntimeState,
    pub resources: ResourceState,
    pub network: NetworkState,
}