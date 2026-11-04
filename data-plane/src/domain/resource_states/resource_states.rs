#[derive(Clone)]
pub struct ResourceStates {
    pub cpu_usage: f32,
    pub memory_usage: u64,
    pub within_limits: bool,
}