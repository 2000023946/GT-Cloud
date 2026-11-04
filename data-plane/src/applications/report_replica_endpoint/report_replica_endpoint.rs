use crate::domain::endpoint::Endpoint;

pub trait ReportReplicaEndpoint {
    fn execute(&self, endpoint: Endpoint) -> Result<(), String>;
}