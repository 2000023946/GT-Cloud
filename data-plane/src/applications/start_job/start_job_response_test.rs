
#[cfg(test)]
mod tests {
    use crate::applications::start_job::start_job_response::StartJobResponse;

use super::*;

    #[test]
    fn test_create_error() {
        let response = StartJobResponse::create_error(
            "Job failed!".to_string()
        );

        assert!(response.is_error);
        assert_eq!(response.message, "Job failed!");
    }

    #[test]
    fn test_create_success() {
        let response = StartJobResponse::create_success(
            "Job started successfully!".to_string()
        );

        assert!(!response.is_error);
        assert_eq!(response.message, "Job started successfully!");
    }

    #[test]
    fn test_create_error_empty_message() {
        let response = StartJobResponse::create_error(
            String::new()
        );

        assert!(response.is_error);
        assert!(response.message.is_empty());
    }

    #[test]
    fn test_create_success_empty_message() {
        let response = StartJobResponse::create_success(
            String::new()
        );

        assert!(!response.is_error);
        assert!(response.message.is_empty());
    }
}
