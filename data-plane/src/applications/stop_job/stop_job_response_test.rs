
#[cfg(test)]
mod tests {
    use crate::applications::stop_job::stop_job_response::StopJobResponse;

use super::*;

    #[test]
    fn test_stop_job_response_success() {
        let response = StopJobResponse::success();

        assert!(!response.is_error);
        assert!(response.is_success);
        assert_eq!(response.message, "");
    }

    #[test]
    fn test_stop_job_response_error() {
        let message = "Failed to stop job".to_string();

        let response = StopJobResponse::error(message.clone());

        assert!(response.is_error);
        assert!(!response.is_success);
        assert_eq!(response.message, message);
    }

    #[test]
    fn test_stop_job_response_error_empty_message() {
        let response = StopJobResponse::error(String::new());

        assert!(response.is_error);
        assert!(!response.is_success);
        assert!(response.message.is_empty());
    }
}
