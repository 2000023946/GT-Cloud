#[cfg(test)]
mod tests {
    use crate::applications::get_job_logs::get_job_logs_response::GetJobLogsResponse;

use super::*;

    #[test]
    fn test_success() {
        let stdout = String::from("Application started");
        let stderr = String::from("Warning: deprecated API");

        let response = GetJobLogsResponse::success(stdout.clone(), stderr.clone());

        assert!(!response.is_error);
        assert_eq!(response.message, "");
        assert_eq!(response.stdout, stdout);
        assert_eq!(response.stderr, stderr);
    }

    #[test]
    fn test_error() {
        let message = String::from("Job not found");

        let response = GetJobLogsResponse::error(message.clone());

        assert!(response.is_error);
        assert_eq!(response.message, message);
        assert_eq!(response.stdout, "");
        assert_eq!(response.stderr, "");
    }

    #[test]
    fn test_success_empty_logs() {
        let response = GetJobLogsResponse::success(
            String::new(),
            String::new(),
        );

        assert!(!response.is_error);
        assert_eq!(response.message, "");
        assert_eq!(response.stdout, "");
        assert_eq!(response.stderr, "");
    }

    #[test]
    fn test_error_empty_message() {
        let response = GetJobLogsResponse::error(String::new());

        assert!(response.is_error);
        assert_eq!(response.message, "");
        assert_eq!(response.stdout, "");
        assert_eq!(response.stderr, "");
    }
}