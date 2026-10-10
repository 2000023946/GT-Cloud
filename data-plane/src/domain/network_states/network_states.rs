#[derive(Clone)]
pub struct NetworkStates {
    pub healthy: bool,
    pub ip_address: Option<String>,
}