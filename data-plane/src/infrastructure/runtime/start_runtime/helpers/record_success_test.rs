
#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    use crate::{
        infrastructure::runtime::start_runtime::helpers::record_success::record_success,
        observability::observability::Observability,
        ports::{logger::Logger, metrics::Metrics},
    };

    #[derive(Clone, Debug)]
    struct LogEntry {
        message: String,
        fields: HashMap<String, String>,
    }

    #[derive(Clone)]
    struct TestLogger {
        entries: Arc<Mutex<Vec<LogEntry>>>,
    }

    impl Logger for TestLogger {
        fn info(&self, message: &str, fields: HashMap<String, String>) {
            self.entries.lock().unwrap().push(LogEntry {
                message: message.to_string(),
                fields,
            });
        }

        fn error(&self, _: &str, _: HashMap<String, String>) {}
    }

    #[derive(Clone, Debug)]
    struct MetricEntry {
        name: String,
        value: f64,
    }

    #[derive(Clone)]
    struct TestMetrics {
        entries: Arc<Mutex<Vec<MetricEntry>>>,
    }

    impl Metrics for TestMetrics {
        fn increment(&self, name: &str, value: f64) {
            self.entries.lock().unwrap().push(MetricEntry {
                name: name.to_string(),
                value,
            });
        }

        fn observe(&self, _: &str, _: f64) {}
    }

    fn create_observability() -> (
        Observability<TestLogger, TestMetrics>,
        Arc<Mutex<Vec<LogEntry>>>,
        Arc<Mutex<Vec<MetricEntry>>>,
    ) {
        let logs = Arc::new(Mutex::new(Vec::new()));
        let metrics = Arc::new(Mutex::new(Vec::new()));

        let observability = Observability {
            logger: TestLogger {
                entries: Arc::clone(&logs),
            },
            metrics: TestMetrics {
                entries: Arc::clone(&metrics),
            },
        };

        (observability, logs, metrics)
    }

    #[test]
    fn records_success_log_with_job_id_and_pid() {
        let (observability, logs, _) = create_observability();

        record_success(&observability, "job-123", 456);

        let logs = logs.lock().unwrap();

        assert_eq!(logs.len(), 1);
        assert_eq!(logs[0].message, "Runtime started job");
        assert_eq!(
            logs[0].fields.get("job_id").map(String::as_str),
            Some("job-123")
        );
        assert_eq!(
            logs[0].fields.get("pid").map(String::as_str),
            Some("456")
        );
        assert_eq!(logs[0].fields.len(), 2);
    }

    #[test]
    fn increments_runtime_start_success_metric() {
        let (observability, _, metrics) = create_observability();

        record_success(&observability, "job-123", 456);

        let metrics = metrics.lock().unwrap();

        assert_eq!(metrics.len(), 1);
        assert_eq!(metrics[0].name, "runtime.start.success");
        assert_eq!(metrics[0].value, 1.0);
    }

    #[test]
    fn records_each_success_independently() {
        let (observability, logs, metrics) = create_observability();

        record_success(&observability, "job-123", 456);
        record_success(&observability, "job-789", 789);

        let logs = logs.lock().unwrap();
        let metrics = metrics.lock().unwrap();

        assert_eq!(logs.len(), 2);
        assert_eq!(metrics.len(), 2);

        assert_eq!(
            logs[0].fields.get("job_id").map(String::as_str),
            Some("job-123")
        );
        assert_eq!(
            logs[1].fields.get("job_id").map(String::as_str),
            Some("job-789")
        );

        assert!(metrics.iter().all(|metric| {
            metric.name == "runtime.start.success" && metric.value == 1.0
        }));
    }

    #[test]
    fn does_not_record_error_logs_on_success() {
        let (observability, logs, _) = create_observability();

        record_success(&observability, "job-123", 456);

        let logs = logs.lock().unwrap();

        assert_eq!(logs.len(), 1);
        assert_eq!(logs[0].message, "Runtime started job");
    }
}
