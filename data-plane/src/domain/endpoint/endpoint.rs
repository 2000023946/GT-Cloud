#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Endpoint {
    pub replica_id: String,
    pub address: String,
    pub port: u16,
}