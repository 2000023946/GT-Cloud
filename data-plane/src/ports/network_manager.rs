use crate::domain::{job::Job, network_states::network_states::NetworkStates};

pub trait NetworkManager {
    fn configure(&self, job: Job) -> Result<(), String>;

    fn monitor(&self, job: Job) -> Result<NetworkStates, String>;

    fn cleanup(&self, job: Job) -> Result<(), String>;
}