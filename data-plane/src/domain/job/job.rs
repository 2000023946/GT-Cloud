use crate::domain::app::App;
use crate::domain::job::JobStatus;
use crate::domain::network::NetworkRule;
use crate::domain::resource::Resource;

#[derive(Clone)]
pub struct Job {
    pub id: String,
    pub app: App,
    pub status: JobStatus,
    pub resources: Resource,
    pub network: Vec<NetworkRule>,
}
