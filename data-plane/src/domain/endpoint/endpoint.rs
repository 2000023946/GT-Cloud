#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Endpoint {
    pub replica_id: String,
    pub address: String,
    pub port: u16,
}

impl Endpoint {
    pub fn new(
        replica_id: String,
        address: String,
        port: u16,
    ) -> Self {
        Self {
            replica_id,
            address,
            port,
        }
    }
}