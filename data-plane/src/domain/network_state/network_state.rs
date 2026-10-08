#[derive(Clone)]
pub struct NetworkState {
    pub healthy: bool,
    pub ip_address: Option<String>,
}