#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        applications::get_job_resources::{get_job_resources_request::GetJobResourcesRequest, get_job_resources_use_case::GetJobResourcesUseCase}, domain::{
            job::Job,
            resource_states::resource_states::ResourceStates,
        }, ports::resource_manager::ResourceManager,
    };

    struct MockResourceManager {
        result: Result<ResourceStates, String>,
        received_job_id: std::sync::Mutex<Option<String>>,
    }

    impl ResourceManager for MockResourceManager {
        fn configure(&self, _job: &Job) -> Result<(), String> {
            Ok(())
        }

        fn start(&self, _job: &Job, _pid: u32) -> Result<(), String> {
            Ok(())
        }

        fn monitor(&self, job_id: String) -> Result<ResourceStates, String> {
            *self.received_job_id.lock().unwrap() = Some(job_id);
            self.result.clone()
        }

        fn cleanup(&self, _job_id: String) -> Result<(), String> {
            Ok(())
        }

        fn stop(&self, job_id: String) -> Result<(), String> {
            Ok(())
        }
    }

    #[test]
    fn test_execute_success() {
        let mock = MockResourceManager {
            result: Ok(ResourceStates {
                cpu_usage: 75.0,
                memory_usage: 1024.0,
                within_limits: true,
            }),
            received_job_id: std::sync::Mutex::new(None),
        };

        let use_case = GetJobResourcesUseCase {
            resource_manager: mock,
        };

        let request = GetJobResourcesRequest {
            job_id: "job-123".to_string(),
        };

        let response = use_case.execute(request);

        assert_eq!(response.cpu_usage, 75.0);
        assert_eq!(response.memory_usage, 1024.0);
        assert!(!response.is_error);
        assert_eq!(response.message, "");

        assert_eq!(
            use_case.resource_manager.received_job_id.lock().unwrap().as_deref(),
            Some("job-123")
        );
    }

    #[test]
    fn test_execute_resource_manager_error() {
        let mock = MockResourceManager {
            result: Err("Failed to monitor job".to_string()),
            received_job_id: std::sync::Mutex::new(None),
        };

        let use_case = GetJobResourcesUseCase {
            resource_manager: mock,
        };

        let request = GetJobResourcesRequest {
            job_id: "job-123".to_string(),
        };

        let response = use_case.execute(request);

        assert_eq!(response.cpu_usage, 0.0);
        assert_eq!(response.memory_usage, 0.0);
        assert!(response.is_error);
        assert_eq!(response.message, "Failed to monitor job");

        assert_eq!(
            use_case.resource_manager.received_job_id.lock().unwrap().as_deref(),
            Some("job-123")
        );
    }
}