pub struct GetJobResourcesResponse {
    pub cpu_usage: f32,
    pub memory_usage: f32,
    pub is_error: bool,
    pub message: String
}

impl GetJobResourcesResponse{
    pub fn success(cpu_usage: f32, memory_usage: f32) -> Self {
        return Self { cpu_usage, memory_usage, is_error: false, message: "".to_string()}
    }
    pub fn error(message: String) -> Self {
        return Self {cpu_usage: 0.0, memory_usage: 0.0, is_error: true, message: message}
    }
}