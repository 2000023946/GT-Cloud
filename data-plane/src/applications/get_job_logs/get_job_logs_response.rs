pub struct GetJobLogsResponse {
    pub is_error: bool,
    pub message: String,
    pub stdout: String,
    pub stderr: String
}

impl GetJobLogsResponse {
    pub fn success(stdout: String, stderr: String) -> Self {
        Self { is_error: false, message: "".to_string(), stdout, stderr }
    }

    pub fn error(message: String) -> Self {
        Self { is_error: true, message, stdout: "".to_string(), stderr: "".to_string() }
    }
}