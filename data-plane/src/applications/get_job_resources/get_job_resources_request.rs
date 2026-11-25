use crate::domain::job;

#[derive(Debug)]
pub struct GetJobResourcesRequest {
    pub job_id: String
}


impl GetJobResourcesRequest {
    pub fn new(job_id: String) -> Result<GetJobResourcesRequest, String> {
        if job_id.trim().is_empty() {
            return Err("Job ID cannot be empty".to_string())
        }
        return Ok(GetJobResourcesRequest{job_id: job_id})
    }
}