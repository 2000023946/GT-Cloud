pub struct StartJobResponse {
    pub is_error: bool,
    pub message: String
} 

impl StartJobResponse {
    pub fn create_error(message: String) -> Self {
        Self {message: message, is_error: true }
    }
    pub fn create_success(message: String) -> Self {
        Self {message: message, is_error: false }
    }
}