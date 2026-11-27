#[cfg(test)]
mod tests {
    use crate::applications::get_job_resources::get_job_resources_request::GetJobResourcesRequest;

use super::*;

    #[test]
    fn test_new_empty_job_id_returns_error() {
        let result = GetJobResourcesRequest::new(String::new());

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Job ID cannot be empty");
    }

    #[test]
    fn test_new_valid_job_id_returns_request() {
        let job_id = "job-123".to_string();

        let result = GetJobResourcesRequest::new(job_id.clone());

        assert!(result.is_ok());

        let request = result.unwrap();
        assert_eq!(request.job_id, job_id);
    }
}