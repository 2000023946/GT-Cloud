use crate::domain::{network_states::network_states::NetworkStates, resource_states::resource_states::ResourceStates, runtime_states::runtime_states::RuntimeStates};



#[derive(Clone)]
pub struct JobState {
    pub job_id: String,
    pub runtime: RuntimeStates,
    pub resources: ResourceStates,
    pub network: NetworkStates,
}