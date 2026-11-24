#[cfg(test)]
mod tests {
    use crate::applications::get_job_logs::get_job_logs_request::GetJobLogsRequest;

use super::*;

    #[test]
    fn test_new_success() {
        let job_id = String::from("job-123");

        let result = GetJobLogsRequest::new(job_id.clone());

        assert!(result.is_ok());

        let request = result.unwrap();
        assert_eq!(request.job_id, job_id);
    }

    #[test]
    fn test_new_empty_job_id() {
        let job_id = String::new();

        let result = GetJobLogsRequest::new(job_id);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "job_id cannot be empty");
    }
}