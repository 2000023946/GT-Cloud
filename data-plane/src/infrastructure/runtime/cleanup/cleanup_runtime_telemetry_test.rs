
#[cfg(test)]
mod tests {
    use crate::{infrastructure::runtime::cleanup::cleanup_runtime_telemetry::CleanupRuntimeTelemetry, observability::observability::Observability, ports::{logger::Logger, metrics::Metrics}};

    use std::{collections::HashMap, sync::{Arc, Mutex}};

    // Mock Logger
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

    // Mock Metrics
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

    fn create_telemetry() -> (
        CleanupRuntimeTelemetry<TestLogger, TestMetrics>,
        TestLogger,
        TestMetrics,
    ) {
        let logger = TestLogger::default();
        let metrics = TestMetrics::default();

        let telemetry = CleanupRuntimeTelemetry::new(
            Observability {
                logger: logger.clone(),
                metrics: metrics.clone(),
            },
        );

        (telemetry, logger, metrics)
    }

    #[test]
    fn log_cleanup_records_success() {
        let (telemetry, logger, metrics) = create_telemetry();

        telemetry.log_cleanup("job-123");

        // Verify success log
        let info_calls = logger.info_calls.lock().unwrap();

        assert_eq!(info_calls.len(), 1);
        assert_eq!(info_calls[0].0, "Successfully cleaned up runtime");
        assert_eq!(
            info_calls[0].1.get("job_id").unwrap(),
            "job-123"
        );

        // Verify success metric
        let increment_calls = metrics.increment_calls.lock().unwrap();

        assert_eq!(increment_calls.len(), 1);
        assert_eq!(
            increment_calls[0],
            ("runtime_cleanup_success".to_string(), 1.0)
        );

        // Ensure no failure was logged
        assert!(logger.error_calls.lock().unwrap().is_empty());
    }

    #[test]
    fn log_failure_records_error() {
        let (telemetry, logger, metrics) = create_telemetry();

        telemetry.log_failure("job-123", "Failed to remove process state");

        // Verify error log
        let error_calls = logger.error_calls.lock().unwrap();

        assert_eq!(error_calls.len(), 1);
        assert_eq!(error_calls[0].0, "Failed to clean up runtime");
        assert_eq!(
            error_calls[0].1.get("job_id").unwrap(),
            "job-123"
        );
        assert_eq!(
            error_calls[0].1.get("error").unwrap(),
            "Failed to remove process state"
        );

        // Verify failure metric
        let increment_calls = metrics.increment_calls.lock().unwrap();

        assert_eq!(increment_calls.len(), 1);
        assert_eq!(
            increment_calls[0],
            ("runtime_cleanup_failure".to_string(), 1.0)
        );

        // Ensure no success was logged
        assert!(logger.info_calls.lock().unwrap().is_empty());
    }
}
