use crate::domain::network::NetworkDirection;

#[derive(Clone)]
pub struct NetworkRule {
    pub protocol: String,
    pub port: i32,
    pub direction: NetworkDirection,
}
