#[cfg(test)]
mod tests {
    use crate::applications::get_job_resources::get_job_resources_response::GetJobResourcesResponse;

    use super::*;

    #[test]
    fn test_success_response() {
        let response = GetJobResourcesResponse::success(75.0_f32, 1024.0_f32);

        assert_eq!(response.cpu_usage, 75.0_f32);
        assert_eq!(response.memory_usage, 1024.0_f32);
        assert!(!response.is_error);
        assert_eq!(response.message, "");
    }

    #[test]
    fn test_error_response() {
        let message = "Failed to retrieve job resources".to_string();

        let response = GetJobResourcesResponse::error(message.clone());

        assert_eq!(response.cpu_usage, 0.0_f32);
        assert_eq!(response.memory_usage, 0.0_f32);
        assert!(response.is_error);
        assert_eq!(response.message, message);
    }

    #[test]
    fn test_success_response_with_zero_usage() {
        let response = GetJobResourcesResponse::success(0.0_f32, 0.0_f32);

        assert_eq!(response.cpu_usage, 0.0_f32);
        assert_eq!(response.memory_usage, 0.0_f32);
        assert!(!response.is_error);
        assert_eq!(response.message, "");
    }

    #[test]
    fn test_error_response_with_empty_message() {
        let response = GetJobResourcesResponse::error(String::new());

        assert_eq!(response.cpu_usage, 0.0_f32);
        assert_eq!(response.memory_usage, 0.0_f32);
        assert!(response.is_error);
        assert!(response.message.is_empty());
    }
}