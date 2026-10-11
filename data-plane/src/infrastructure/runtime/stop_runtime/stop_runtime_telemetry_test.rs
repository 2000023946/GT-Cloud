
#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        collections::HashMap,
        sync::Mutex,
    };

    use crate::{
        infrastructure::runtime::stop_runtime::stop_runtime_telemetry::StopRuntimeTelemetry, observability::observability::Observability, ports::{logger::Logger, metrics::Metrics},
    };

    #[derive(Debug, PartialEq)]
    struct LogCall {
        level: String,
        message: String,
        fields: HashMap<String, String>,
    }

    #[derive(Debug, PartialEq)]
    struct MetricCall {
        operation: String,
        name: String,
        value: f64,
    }

    #[derive(Default)]
    struct MockLogger {
        calls: Mutex<Vec<LogCall>>,
    }

    impl Logger for MockLogger {
        fn info(
            &self,
            message: &str,
            fields: HashMap<String, String>,
        ) {
            self.calls.lock().unwrap().push(LogCall {
                level: "info".to_string(),
                message: message.to_string(),
                fields,
            });
        }

        fn error(
            &self,
            message: &str,
            fields: HashMap<String, String>,
        ) {
            self.calls.lock().unwrap().push(LogCall {
                level: "error".to_string(),
                message: message.to_string(),
                fields,
            });
        }
    }

    #[derive(Default)]
    struct MockMetrics {
        calls: Mutex<Vec<MetricCall>>,
    }

    impl Metrics for MockMetrics {
        fn increment(&self, name: &str, value: f64) {
            self.calls.lock().unwrap().push(MetricCall {
                operation: "increment".to_string(),
                name: name.to_string(),
                value,
            });
        }

        fn observe(&self, name: &str, value: f64) {
            self.calls.lock().unwrap().push(MetricCall {
                operation: "observe".to_string(),
                name: name.to_string(),
                value,
            });
        }
    }

    fn create_telemetry(
    ) -> StopRuntimeTelemetry<MockLogger, MockMetrics> {
        StopRuntimeTelemetry::new(Observability {
            logger: MockLogger::default(),
            metrics: MockMetrics::default(),
        })
    }

    #[test]
    fn log_stop_success_records_message_and_fields() {
        let telemetry = create_telemetry();

        telemetry.log_stop_success("job-123", 42);

        let calls = telemetry.observability.logger.calls.lock().unwrap();

        assert_eq!(calls.len(), 1);
        assert_eq!(
            calls[0],
            LogCall {
                level: "info".to_string(),
                message: "SIGTERM sent successfully".to_string(),
                fields: HashMap::from([
                    ("job_id".to_string(), "job-123".to_string()),
                    ("pid".to_string(), "42".to_string()),
                ]),
            }
        );
    }

    #[test]
    fn log_stop_failure_records_message_and_fields() {
        let telemetry = create_telemetry();

        telemetry.log_stop_failure(
            "job-456",
            "Permission denied when stopping process",
        );

        let calls = telemetry.observability.logger.calls.lock().unwrap();

        assert_eq!(calls.len(), 1);
        assert_eq!(
            calls[0],
            LogCall {
                level: "error".to_string(),
                message: "Failed to stop job".to_string(),
                fields: HashMap::from([
                    ("job_id".to_string(), "job-456".to_string()),
                    (
                        "error".to_string(),
                        "Permission denied when stopping process".to_string(),
                    ),
                ]),
            }
        );
    }

    #[test]
    fn record_stop_success_increments_success_metric() {
        let telemetry = create_telemetry();

        telemetry.record_stop_success();

        let calls = telemetry.observability.metrics.calls.lock().unwrap();

        assert_eq!(
            *calls,
            vec![MetricCall {
                operation: "increment".to_string(),
                name: "runtime.stop.successes".to_string(),
                value: 1.0,
            }]
        );
    }

    #[test]
    fn record_stop_failure_increments_failure_metric() {
        let telemetry = create_telemetry();

        telemetry.record_stop_failure();

        let calls = telemetry.observability.metrics.calls.lock().unwrap();

        assert_eq!(
            *calls,
            vec![MetricCall {
                operation: "increment".to_string(),
                name: "runtime.stop.failures".to_string(),
                value: 1.0,
            }]
        );
    }
}
