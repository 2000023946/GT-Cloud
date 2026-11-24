use crate::domain::app::App;
use crate::domain::job::JobStatus;
use crate::domain::network::NetworkRule;
use crate::domain::resource::Resource;

#[derive(Clone)]
pub struct Job {
    pub id: String,
    pub app: App,
    pub name: String,
    pub status: JobStatus,
    pub resources: Resource,
    pub network: Vec<NetworkRule>,
}


impl Job {
    pub fn new(
        id: String,
        name: String,
        cpu: f32,
        memory: f32,
        path: String,
        command: String,
    ) -> Self {
        let app = App { 
            id: id.clone(), code_path: path, command: command, environment: vec![] 
        };
        let resources:Resource = Resource {
            cpu: cpu, memory: memory
        };
        Self {
            id, app, name, resources, status: JobStatus::Starting,network: vec![]
        }
    }
}