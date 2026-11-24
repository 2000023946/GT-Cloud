#[derive(Debug)]
pub struct GetJobLogsRequest {
    pub job_id: String
}

impl GetJobLogsRequest {
    pub fn new(job_id: String) -> Result<Self, String> {
        if job_id.trim().is_empty() {
            return Err("job_id cannot be empty".to_string());
        }
        Ok(Self {job_id})
    }
}