pub struct StopJobResponse {
    pub is_error: bool,
    pub is_success: bool,
    pub message: String
}

impl StopJobResponse {
    pub fn success() -> Self {
        Self { is_error: false, is_success: true, message: "".to_string() }
    }
    pub fn error(message: String) -> Self {
        Self { is_error: true, is_success: false, message: message }
    }
}