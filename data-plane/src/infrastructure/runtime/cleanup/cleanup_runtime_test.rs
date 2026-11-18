#[cfg(test)]
mod tests {
    use crate::{
        domain::{
            app::App,
            job::{Job, JobStatus},
            network::NetworkRule,
            resource::Resource,
        },
        infrastructure::runtime::{
            cleanup::{
                cleanup_runtime::CleanupRuntime,
                cleanup_runtime_telemetry::CleanupRuntimeTelemetry,
            },
            states::{
                process_state::ProcessState,
                runtime_state::RuntimeState,
            },
        },
        observability::observability::Observability,
        ports::{logger::Logger, metrics::Metrics},
    };

    use std::{
        collections::HashMap,
        sync::{Arc, Mutex},
    };

    // --------------------------------------------------
    // Mock Logger
    // --------------------------------------------------

    #[derive(Clone, Default)]
    struct TestLogger {
        info_calls: Arc<Mutex<Vec<(String, HashMap<String, String>)>>>,
        error_calls: Arc<Mutex<Vec<(String, HashMap<String, String>)>>>,
    }

    impl Logger for TestLogger {
        fn info(&self, message: &str, fields: HashMap<String, String>) {
            self.info_calls
                .lock()
                .unwrap()
                .push((message.to_string(), fields));
        }

        fn error(&self, message: &str, fields: HashMap<String, String>) {
            self.error_calls
                .lock()
                .unwrap()
                .push((message.to_string(), fields));
        }
    }

    // --------------------------------------------------
    // Mock Metrics
    // --------------------------------------------------

    #[derive(Clone, Default)]
    struct TestMetrics {
        increment_calls: Arc<Mutex<Vec<(String, f64)>>>,
        observe_calls: Arc<Mutex<Vec<(String, f64)>>>,
    }

    impl Metrics for TestMetrics {
        fn increment(&self, name: &str, value: f64) {
            self.increment_calls
                .lock()
                .unwrap()
                .push((name.to_string(), value));
        }

        fn observe(&self, name: &str, value: f64) {
            self.observe_calls
                .lock()
                .unwrap()
                .push((name.to_string(), value));
        }
    }

    // --------------------------------------------------
    // Test Helpers
    // --------------------------------------------------

    pub(crate) fn create_job(job_id: &str) -> Job {
        Job {
            id: job_id.to_string(),
            app: App {
                id: "app-123".to_string(),
                code_path: "/tmp/app".to_string(),
                command: "python app.py".to_string(),
                environment: vec![],
            },
            name: "test-job".to_string(),
            status: JobStatus::Stopped,
            resources: Resource {
                cpu: 1,
                memory: 512,
            },
            network: Vec::<NetworkRule>::new(),
        }
    }

    pub(crate) fn create_cleanup_runtime() -> (
        CleanupRuntime<TestLogger, TestMetrics>,
        Arc<RuntimeState>,
        TestLogger,
        TestMetrics,
    ) {
        let runtime_state = Arc::new(RuntimeState::new());

        let logger = TestLogger::default();
        let metrics = TestMetrics::default();

        let observability = Observability {
            logger: logger.clone(),
            metrics: metrics.clone(),
        };

        let telemetry = CleanupRuntimeTelemetry::new(observability);

        let cleanup = CleanupRuntime::new(telemetry);

        (cleanup, runtime_state, logger, metrics)
    }

    // --------------------------------------------------
    // Successful Cleanup Tests
    // --------------------------------------------------

    #[test]
    fn cleanup_removes_process() {
        let (cleanup, runtime_state, _, _) = create_cleanup_runtime();

        let job_id = "job-123";

        runtime_state.add_process(
            job_id.to_string(),
            ProcessState {
                program: "python".to_string(),
                args: vec!["app.py".to_string()],
                working_directory: Some("/tmp/app".to_string()),
                environment: vec![],
                pid: Some(1234),
            },
        );

        assert!(runtime_state.contains_process(job_id));

        cleanup
            .cleanup(&runtime_state, create_job(job_id))
            .unwrap();

        assert!(!runtime_state.contains_process(job_id));
    }

    #[test]
    fn cleanup_removes_stack() {
        let (cleanup, runtime_state, _, _) = create_cleanup_runtime();

        let job_id = "job-123";

        runtime_state.add_stack(
            job_id.to_string(),
            vec![0u8; 4096],
        );

        assert!(runtime_state.contains_stack(job_id));

        cleanup
            .cleanup(&runtime_state, create_job(job_id))
            .unwrap();

        assert!(!runtime_state.contains_stack(job_id));
    }

    #[test]
    fn cleanup_removes_process_and_stack() {
        let (cleanup, runtime_state, _, _) = create_cleanup_runtime();

        let job_id = "job-123";

        runtime_state.add_process(
            job_id.to_string(),
            ProcessState {
                program: "python".to_string(),
                args: vec!["app.py".to_string()],
                working_directory: Some("/tmp/app".to_string()),
                environment: vec![],
                pid: Some(1234),
            },
        );

        runtime_state.add_stack(
            job_id.to_string(),
            vec![0u8; 4096],
        );

        cleanup
            .cleanup(&runtime_state, create_job(job_id))
            .unwrap();

        assert!(!runtime_state.contains_process(job_id));
        assert!(!runtime_state.contains_stack(job_id));
    }

    #[test]
    fn cleanup_logs_success() {
        let (cleanup, runtime_state, logger, metrics) =
            create_cleanup_runtime();

        cleanup
            .cleanup(&runtime_state, create_job("job-123"))
            .unwrap();

        let info_calls = logger.info_calls.lock().unwrap();

        assert_eq!(info_calls.len(), 1);
        assert_eq!(
            info_calls[0].1.get("job_id").unwrap(),
            "job-123"
        );

        let increment_calls = metrics.increment_calls.lock().unwrap();

        assert_eq!(increment_calls.len(), 1);
        assert_eq!(
            increment_calls[0],
            ("runtime_cleanup_success".to_string(), 1.0)
        );

        assert!(logger.error_calls.lock().unwrap().is_empty());
    }

    #[test]
    fn cleanup_can_run_multiple_times() {
        let (cleanup, runtime_state, logger, metrics) =
            create_cleanup_runtime();

        let job_id = "job-123";

        runtime_state.add_stack(
            job_id.to_string(),
            vec![0u8; 4096],
        );

        cleanup
            .cleanup(&runtime_state, create_job(job_id))
            .unwrap();

        cleanup
            .cleanup(&runtime_state, create_job(job_id))
            .unwrap();

        assert!(!runtime_state.contains_stack(job_id));
        assert_eq!(logger.info_calls.lock().unwrap().len(), 2);

        let increment_calls = metrics.increment_calls.lock().unwrap();

        assert_eq!(increment_calls.len(), 2);
    }
}