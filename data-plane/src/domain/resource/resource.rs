use crate::domain::globals::globals::MAX_CPU;
use crate::domain::globals::globals::MAX_MEMORY;
#[derive(Clone)]
pub struct Resource {
    pub cpu: f32,
    pub memory: f32,
}

impl Resource {
    pub fn new(
        cpu: f32,
        memory: f32
    ) -> Result<Self, String> {
        if cpu <= 0.0 {
            return Err("CPU cannot be negative!".to_string());
        }
        if cpu >= MAX_CPU as f32 {
            return Err("CPU too big!".to_string());
        }
        if memory <= 0.0 {
            return Err("Memory cannot be negative!".to_string());
        }
        if memory >= MAX_MEMORY as f32 {
            return Err("Memory too big!".to_string());
        }
        Ok(Self {cpu: cpu, memory: memory})
    }
}