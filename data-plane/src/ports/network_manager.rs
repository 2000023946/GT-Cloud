use crate::domain::{job::Job, network_state::network_state::NetworkState};

pub trait NetworkManager {
    fn configure(&self, job: Job) -> Result<(), String>;

    fn monitor(&self, job: Job) -> Result<NetworkState, String>;

    fn cleanup(&self, job: Job) -> Result<(), String>;
}