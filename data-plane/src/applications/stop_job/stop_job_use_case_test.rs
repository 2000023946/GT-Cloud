
#[cfg(test)]
mod tests {
    use crate::{applications::stop_job::{stop_job_request::StopJobRequest, stop_job_use_case::StopJobUseCase}, domain::{job::Job, resource_states::resource_states::ResourceStates, runtime_states::runtime_states::RuntimeStates}, ports::{resource_manager::ResourceManager, runtime::Runtime}};

use super::*;
    use std::sync::Mutex;

    // -----------------------------
    // Runtime mock
    // -----------------------------

    struct MockRuntime {
        stop_error: Option<String>,
        cleanup_error: Option<String>,
        calls: Mutex<Vec<String>>,
    }

    impl MockRuntime {
        fn new(
            stop_error: Option<String>,
            cleanup_error: Option<String>,
        ) -> Self {
            Self {
                stop_error,
                cleanup_error,
                calls: Mutex::new(Vec::new()),
            }
        }
    }

    impl Runtime for MockRuntime {
        fn configure(&self, _job: &Job) -> Result<(), String> {
            unreachable!("configure should not be called")
        }

        fn get_pid(&self, _job: &Job) -> Result<u32, String> {
            unreachable!("get_pid should not be called")
        }

        fn start(&self, _job: &Job) -> Result<(), String> {
            unreachable!("start should not be called")
        }

        fn monitor(&self, _job: &Job) -> Result<RuntimeStates, String> {
            unreachable!("monitor should not be called")
        }

        fn stop(&self, job_id: String) -> Result<(), String> {
            self.calls.lock().unwrap().push(format!("stop:{job_id}"));

            match &self.stop_error {
                Some(error) => Err(error.clone()),
                None => Ok(()),
            }
        }

        fn cleanup(&self, job_id: String) -> Result<(), String> {
            self.calls.lock().unwrap().push(format!("cleanup:{job_id}"));

            match &self.cleanup_error {
                Some(error) => Err(error.clone()),
                None => Ok(()),
            }
        }
    }

    // -----------------------------
    // Resource manager mock
    // -----------------------------

    struct MockResourceManager {
        stop_error: Option<String>,
        cleanup_error: Option<String>,
        calls: Mutex<Vec<String>>,
    }

    impl MockResourceManager {
        fn new(
            stop_error: Option<String>,
            cleanup_error: Option<String>,
        ) -> Self {
            Self {
                stop_error,
                cleanup_error,
                calls: Mutex::new(Vec::new()),
            }
        }
    }

    impl ResourceManager for MockResourceManager {
        fn configure(&self, _job: &Job) -> Result<(), String> {
            unreachable!("configure should not be called")
        }

        fn start(&self, _job: &Job, _pid: u32) -> Result<(), String> {
            unreachable!("start should not be called")
        }

        fn monitor(&self, _job_id: String) -> Result<ResourceStates, String> {
            unreachable!("monitor should not be called")
        }

        fn stop(&self, job_id: String) -> Result<(), String> {
            self.calls.lock().unwrap().push(format!("stop:{job_id}"));

            match &self.stop_error {
                Some(error) => Err(error.clone()),
                None => Ok(()),
            }
        }

        fn cleanup(&self, job_id: String) -> Result<(), String> {
            self.calls.lock().unwrap().push(format!("cleanup:{job_id}"));

            match &self.cleanup_error {
                Some(error) => Err(error.clone()),
                None => Ok(()),
            }
        }
    }

    // -----------------------------
    // Success path
    // -----------------------------

    #[test]
    fn test_stop_job_success() {
        let runtime = MockRuntime::new(None, None);
        let resource_manager = MockResourceManager::new(None, None);

        let use_case = StopJobUseCase {
            runtime_manager: runtime,
            resource_manager,
        };

        let request = StopJobRequest {
            job_id: "job-123".to_string(),
        };

        let response = use_case.execute(request);

        // Assert response indicates success.
        assert!(response.is_success);
        assert!(!response.is_error);

        // Assert runtime operations happened in order.
        assert_eq!(
            *use_case.runtime_manager.calls.lock().unwrap(),
            vec![
                "stop:job-123",
                "cleanup:job-123"
            ]
        );

        // Assert resource manager operations happened in order.
        assert_eq!(
            *use_case.resource_manager.calls.lock().unwrap(),
            vec![
                "stop:job-123",
                "cleanup:job-123"
            ]
        );
    }

    // -----------------------------
    // Runtime stop failure
    // -----------------------------

    #[test]
    fn test_stop_job_runtime_stop_error() {
        let runtime = MockRuntime::new(
            Some("runtime stop failed".to_string()),
            None,
        );

        let resource_manager = MockResourceManager::new(None, None);

        let use_case = StopJobUseCase {
            runtime_manager: runtime,
            resource_manager,
        };

        let request = StopJobRequest {
            job_id: "job-123".to_string(),
        };

        let response = use_case.execute(request);

        assert!(response.is_error);
        assert!(!response.is_success);

        // Runtime cleanup must not execute.
        assert_eq!(
            *use_case.runtime_manager.calls.lock().unwrap(),
            vec!["stop:job-123"]
        );

        // Resource manager must not execute.
        assert!(
            use_case.resource_manager.calls.lock().unwrap().is_empty()
        );
    }

    // -----------------------------
    // Runtime cleanup failure
    // -----------------------------

    #[test]
    fn test_stop_job_runtime_cleanup_error() {
        let runtime = MockRuntime::new(
            None,
            Some("runtime cleanup failed".to_string()),
        );

        let resource_manager = MockResourceManager::new(None, None);

        let use_case = StopJobUseCase {
            runtime_manager: runtime,
            resource_manager,
        };

        let request = StopJobRequest {
            job_id: "job-123".to_string(),
        };

        let response = use_case.execute(request);

        assert!(response.is_error);
        assert!(!response.is_success);

        // Runtime stop succeeds, cleanup fails.
        assert_eq!(
            *use_case.runtime_manager.calls.lock().unwrap(),
            vec![
                "stop:job-123",
                "cleanup:job-123"
            ]
        );

        // Resource manager must not execute.
        assert!(
            use_case.resource_manager.calls.lock().unwrap().is_empty()
        );
    }

    // -----------------------------
    // Resource manager stop failure
    // -----------------------------

    #[test]
    fn test_stop_job_resource_stop_error() {
        let runtime = MockRuntime::new(None, None);

        let resource_manager = MockResourceManager::new(
            Some("resource stop failed".to_string()),
            None,
        );

        let use_case = StopJobUseCase {
            runtime_manager: runtime,
            resource_manager,
        };

        let request = StopJobRequest {
            job_id: "job-123".to_string(),
        };

        let response = use_case.execute(request);

        assert!(response.is_error);
        assert!(!response.is_success);

        // Runtime operations should both complete.
        assert_eq!(
            *use_case.runtime_manager.calls.lock().unwrap(),
            vec![
                "stop:job-123",
                "cleanup:job-123"
            ]
        );

        // Resource cleanup must not execute.
        assert_eq!(
            *use_case.resource_manager.calls.lock().unwrap(),
            vec!["stop:job-123"]
        );
    }

    // -----------------------------
    // Resource manager cleanup failure
    // -----------------------------

    #[test]
    fn test_stop_job_resource_cleanup_error() {
        let runtime = MockRuntime::new(None, None);

        let resource_manager = MockResourceManager::new(
            None,
            Some("resource cleanup failed".to_string()),
        );

        let use_case = StopJobUseCase {
            runtime_manager: runtime,
            resource_manager,
        };

        let request = StopJobRequest {
            job_id: "job-123".to_string(),
        };

        let response = use_case.execute(request);

        assert!(response.is_error);
        assert!(!response.is_success);

        // All four operations should have been attempted.
        assert_eq!(
            *use_case.runtime_manager.calls.lock().unwrap(),
            vec![
                "stop:job-123",
                "cleanup:job-123"
            ]
        );

        assert_eq!(
            *use_case.resource_manager.calls.lock().unwrap(),
            vec![
                "stop:job-123",
                "cleanup:job-123"
            ]
        );
    }
}
