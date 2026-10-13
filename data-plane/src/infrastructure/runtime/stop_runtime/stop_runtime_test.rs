
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        domain::{
            app::App, job::{Job, JobStatus}, resource::Resource,
        }, infrastructure::runtime::{
            states::{process_state::ProcessState, runtime_state::RuntimeState}, stop_runtime::{stop_runtime::StopRuntime, stop_runtime_os_adapter::StopRuntimeOsAdapter, stop_runtime_telemetry::StopRuntimeTelemetry},
        }, observability::observability::Observability, ports::{logger::Logger, metrics::Metrics},
    };
    use std::{
        collections::HashMap,
        io,
        sync::{Arc, Mutex},
    };

    type LogRecord = (String, HashMap<String, String>);
    type MetricRecord = (String, f64);

    #[derive(Clone, Default)]
    struct MockLogger {
        infos: Arc<Mutex<Vec<LogRecord>>>,
        errors: Arc<Mutex<Vec<LogRecord>>>,
    }

    impl Logger for MockLogger {
        fn info(&self, message: &str, fields: HashMap<String, String>) {
            self.infos
                .lock()
                .unwrap()
                .push((message.to_string(), fields));
        }

        fn error(&self, message: &str, fields: HashMap<String, String>) {
            self.errors
                .lock()
                .unwrap()
                .push((message.to_string(), fields));
        }
    }

    #[derive(Clone, Default)]
    struct MockMetrics {
        increments: Arc<Mutex<Vec<MetricRecord>>>,
        observations: Arc<Mutex<Vec<MetricRecord>>>,
    }

    impl Metrics for MockMetrics {
        fn increment(&self, name: &str, value: f64) {
            self.increments
                .lock()
                .unwrap()
                .push((name.to_string(), value));
        }

        fn observe(&self, name: &str, value: f64) {
            self.observations
                .lock()
                .unwrap()
                .push((name.to_string(), value));
        }
    }

    struct MockStopRuntimeOsAdapter {
        result: libc::c_int,
        error_code: libc::c_int,
        kill_calls: Arc<Mutex<Vec<(libc::pid_t, libc::c_int)>>>,
        error_calls: Arc<Mutex<usize>>,
    }

    impl StopRuntimeOsAdapter for MockStopRuntimeOsAdapter {
        fn kill(
            &self,
            pid: libc::pid_t,
            signal: libc::c_int,
        ) -> libc::c_int {
            self.kill_calls.lock().unwrap().push((pid, signal));
            self.result
        }

        fn last_os_error(&self) -> io::Error {
            *self.error_calls.lock().unwrap() += 1;
            io::Error::from_raw_os_error(self.error_code)
        }
    }

    type TestStopRuntime =
        StopRuntime<MockStopRuntimeOsAdapter, MockLogger, MockMetrics>;

    struct TestContext {
        runtime: TestStopRuntime,
        logger: MockLogger,
        metrics: MockMetrics,
        kill_calls: Arc<Mutex<Vec<(libc::pid_t, libc::c_int)>>>,
        error_calls: Arc<Mutex<usize>>,
    }

    // process_pid:
    // None          => no process registered
    // Some(None)    => process registered without a PID
    // Some(Some(pid)) => process registered with a PID
    fn setup(
        process_pid: Option<Option<u32>>,
        result: libc::c_int,
        error_code: libc::c_int,
    ) -> TestContext {
        let runtime_state = RuntimeState::new();
        let job_id = "job-1".to_string();

        if let Some(pid) = process_pid {
            runtime_state.add_process(
                job_id.clone(),
                ProcessState {
                    program: "test-program".to_string(),
                    args: vec![],
                    working_directory: None,
                    environment: vec![],
                    pid,
                },
            );
        }

        let logger = MockLogger::default();
        let metrics = MockMetrics::default();

        let telemetry = StopRuntimeTelemetry::new(Observability {
            logger: logger.clone(),
            metrics: metrics.clone(),
        });

        let kill_calls = Arc::new(Mutex::new(Vec::new()));
        let error_calls = Arc::new(Mutex::new(0));

        let os_adapter = MockStopRuntimeOsAdapter {
            result,
            error_code,
            kill_calls: Arc::clone(&kill_calls),
            error_calls: Arc::clone(&error_calls),
        };

        let runtime =
            StopRuntime::new(runtime_state, telemetry, os_adapter);

        TestContext {
            runtime,
            logger,
            metrics,
            kill_calls,
            error_calls,
        }
    }

    fn make_job() -> Job {
        Job {
            id: "job-1".to_string(),
            app: App {
                id: "app-1".to_string(),
                code_path: "/tmp/test".to_string(),
                command: "test-program".to_string(),
                environment: vec![],
            },
            name: "test-job".to_string(),
            status: JobStatus::Running,
            resources: Resource {
                cpu: 1,
                memory: 1024,
            },
            network: vec![],
        }
    }

    fn assert_failure_telemetry(
        context: &TestContext,
        expected_error: &str,
    ) {
        let errors = context.logger.errors.lock().unwrap();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].0, "Failed to stop job");
        assert_eq!(errors[0].1.get("job_id").unwrap(), "job-1");
        assert_eq!(errors[0].1.get("error").unwrap(), expected_error);
        drop(errors);

        assert_eq!(
            *context.metrics.increments.lock().unwrap(),
            vec![("runtime.stop.failures".to_string(), 1.0)]
        );

        assert!(
            context.logger.infos.lock().unwrap().is_empty()
        );
    }

    #[test]
    fn test_stop_success() {
        let context = setup(Some(Some(1234)), 0, libc::ESRCH);

        let result = context.runtime.stop(make_job());

        assert!(result.is_ok());

        assert_eq!(
            *context.kill_calls.lock().unwrap(),
            vec![(1234, libc::SIGTERM)]
        );
        assert_eq!(*context.error_calls.lock().unwrap(), 0);

        let infos = context.logger.infos.lock().unwrap();
        assert_eq!(infos.len(), 1);
        assert_eq!(infos[0].0, "SIGTERM sent successfully");
        assert_eq!(infos[0].1.get("job_id").unwrap(), "job-1");
        assert_eq!(infos[0].1.get("pid").unwrap(), "1234");
        drop(infos);

        assert!(
            context.logger.errors.lock().unwrap().is_empty()
        );

        assert_eq!(
            *context.metrics.increments.lock().unwrap(),
            vec![("runtime.stop.successes".to_string(), 1.0)]
        );
    }

    #[test]
    fn test_stop_fails_when_process_does_not_exist() {
        let context = setup(None, 0, libc::ESRCH);

        let result = context.runtime.stop(make_job());

        assert_eq!(
            result.unwrap_err(),
            "No process found for job job-1"
        );

        assert!(context.kill_calls.lock().unwrap().is_empty());
        assert_eq!(*context.error_calls.lock().unwrap(), 0);

        assert_failure_telemetry(
            &context,
            "No process found for job job-1",
        );
    }

    #[test]
    fn test_stop_fails_when_pid_is_missing() {
        let context = setup(Some(None), 0, libc::ESRCH);

        let result = context.runtime.stop(make_job());

        assert_eq!(
            result.unwrap_err(),
            "No PID found for job job-1"
        );

        assert!(context.kill_calls.lock().unwrap().is_empty());
        assert_eq!(*context.error_calls.lock().unwrap(), 0);

        assert_failure_telemetry(
            &context,
            "No PID found for job job-1",
        );
    }

    #[test]
    fn test_stop_fails_when_pid_is_invalid() {
        let context = setup(
            Some(Some(u32::MAX)),
            0,
            libc::ESRCH,
        );

        let result = context.runtime.stop(make_job());

        assert_eq!(
            result.unwrap_err(),
            "Invalid PID for job job-1"
        );

        assert!(context.kill_calls.lock().unwrap().is_empty());
        assert_eq!(*context.error_calls.lock().unwrap(), 0);

        assert_failure_telemetry(
            &context,
            "Invalid PID for job job-1",
        );
    }

    #[test]
    fn test_stop_fails_when_process_does_not_exist_at_os_level() {
        let context = setup(
            Some(Some(1234)),
            -1,
            libc::ESRCH,
        );

        let result = context.runtime.stop(make_job());

        assert_eq!(
            result.unwrap_err(),
            "Process does not exist"
        );

        assert_eq!(
            *context.kill_calls.lock().unwrap(),
            vec![(1234, libc::SIGTERM)]
        );
        assert_eq!(*context.error_calls.lock().unwrap(), 1);

        assert_failure_telemetry(
            &context,
            "Process does not exist",
        );
    }

    #[test]
    fn test_stop_fails_when_permission_is_denied() {
        let context = setup(
            Some(Some(1234)),
            -1,
            libc::EPERM,
        );

        let result = context.runtime.stop(make_job());

        assert_eq!(
            result.unwrap_err(),
            "Permission denied when stopping process"
        );

        assert_eq!(*context.error_calls.lock().unwrap(), 1);

        assert_failure_telemetry(
            &context,
            "Permission denied when stopping process",
        );
    }

    #[test]
    fn test_stop_fails_with_generic_os_error() {
        let context = setup(
            Some(Some(1234)),
            -1,
            libc::EIO,
        );

        let result = context.runtime.stop(make_job());
        let message = result.unwrap_err();

        assert!(message.starts_with("Failed to stop process:"));
        assert!(message.contains("os error"));

        assert_eq!(*context.error_calls.lock().unwrap(), 1);

        let errors = context.logger.errors.lock().unwrap();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].1.get("error").unwrap(), &message);
        drop(errors);

        assert_eq!(
            *context.metrics.increments.lock().unwrap(),
            vec![("runtime.stop.failures".to_string(), 1.0)]
        );
    }
}
