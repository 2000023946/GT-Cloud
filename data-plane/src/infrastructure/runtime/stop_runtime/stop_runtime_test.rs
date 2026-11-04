
#[cfg(test)]
mod tests {

    use std::{
        collections::{HashMap, VecDeque},
        io,
        sync::{Arc, Mutex},
        time::Duration,
    };

    use crate::{
        domain::{app::App, job::{Job, JobStatus}, resource::Resource}, infrastructure::runtime::{states::{process_state::ProcessState, runtime_state::RuntimeState}, stop_runtime::{stop_runtime::StopRuntime, stop_runtime_os_adapter::StopRuntimeOsAdapter, stop_runtime_telemetry::StopRuntimeTelemetry}}, observability::observability::Observability, ports::{logger::Logger, metrics::Metrics},
    };

    // ============================================================
    // MOCK LOGGER
    // ============================================================

    #[derive(Clone)]
    struct MockLogger {
        info_calls: Arc<Mutex<Vec<(String, HashMap<String, String>)>>>,
        error_calls: Arc<Mutex<Vec<(String, HashMap<String, String>)>>>,
    }

    impl MockLogger {
        fn new() -> Self {
            Self {
                info_calls: Arc::new(Mutex::new(Vec::new())),
                error_calls: Arc::new(Mutex::new(Vec::new())),
            }
        }
    }

    impl Logger for MockLogger {
        fn info(
            &self,
            message: &str,
            fields: HashMap<String, String>,
        ) {
            self.info_calls
                .lock()
                .unwrap()
                .push((message.to_string(), fields));
        }

        fn error(
            &self,
            message: &str,
            fields: HashMap<String, String>,
        ) {
            self.error_calls
                .lock()
                .unwrap()
                .push((message.to_string(), fields));
        }
    }

    // ============================================================
    // MOCK METRICS
    // ============================================================

    #[derive(Clone)]
    struct MockMetrics {
        increment_calls: Arc<Mutex<Vec<(String, f64)>>>,
        observe_calls: Arc<Mutex<Vec<(String, f64)>>>,
    }

    impl MockMetrics {
        fn new() -> Self {
            Self {
                increment_calls: Arc::new(Mutex::new(Vec::new())),
                observe_calls: Arc::new(Mutex::new(Vec::new())),
            }
        }
    }

    impl Metrics for MockMetrics {
        fn increment(
            &self,
            name: &str,
            value: f64,
        ) {
            self.increment_calls
                .lock()
                .unwrap()
                .push((name.to_string(), value));
        }

        fn observe(
            &self,
            name: &str,
            value: f64,
        ) {
            self.observe_calls
                .lock()
                .unwrap()
                .push((name.to_string(), value));
        }
    }

    // ============================================================
    // MOCK OS ADAPTER
    // ============================================================

    struct MockOsAdapter {
        // (return value, errno if return value == -1)
        kill_results: Mutex<VecDeque<(libc::c_int, i32)>>,

        // (return value, errno if return value == -1)
        wait_results: Mutex<VecDeque<(libc::pid_t, i32)>>,

        last_errno: Mutex<i32>,

        kill_calls: Mutex<Vec<(libc::pid_t, libc::c_int)>>,
        wait_calls: Mutex<Vec<(libc::pid_t, libc::c_int)>>,
    }

    impl MockOsAdapter {
        fn new(
            kill_results: Vec<(libc::c_int, i32)>,
            wait_results: Vec<(libc::pid_t, i32)>,
        ) -> Self {
            Self {
                kill_results: Mutex::new(kill_results.into()),
                wait_results: Mutex::new(wait_results.into()),
                last_errno: Mutex::new(0),
                kill_calls: Mutex::new(Vec::new()),
                wait_calls: Mutex::new(Vec::new()),
            }
        }
    }

    impl StopRuntimeOsAdapter for MockOsAdapter {
        fn kill(
            &self,
            pid: libc::pid_t,
            signal: libc::c_int,
        ) -> libc::c_int {
            self.kill_calls
                .lock()
                .unwrap()
                .push((pid, signal));

            let (result, errno) = self
                .kill_results
                .lock()
                .unwrap()
                .pop_front()
                .unwrap_or((0, 0));

            if result == -1 {
                *self.last_errno.lock().unwrap() = errno;
            }

            result
        }

        fn waitpid(
            &self,
            pid: libc::pid_t,
            _status: &mut libc::c_int,
            options: libc::c_int,
        ) -> libc::pid_t {
            self.wait_calls
                .lock()
                .unwrap()
                .push((pid, options));

            let mut results = self.wait_results.lock().unwrap();

            let (result, errno) = results
                .pop_front()
                .unwrap_or((0, 0));

            if result == -1 {
                *self.last_errno.lock().unwrap() = errno;
            }

            result
        }

        fn sleep(&self, duration: Duration) {
            std::thread::sleep(duration);
        }

        fn last_os_error(&self) -> io::Error {
            io::Error::from_raw_os_error(
                *self.last_errno.lock().unwrap(),
            )
        }
    }

    // ============================================================
    // TEST HELPERS
    // ============================================================

    const JOB_ID: &str = "test-job";
    const PID: u32 = 1234;

    fn create_job() -> Job {
        Job {
            id: JOB_ID.to_string(),
            app: App {
                id: "test-app".to_string(),
                code_path: "/tmp/app".to_string(),
                command: "sleep 5".to_string(),
                environment: vec![],
            },
            name: "test-job".to_string(),
            status: JobStatus::Running,
            resources: Resource {
                cpu: 1,
                memory: 128,
            },
            network: vec![],
        }
    }

    fn create_runtime_state(pid: Option<u32>) -> RuntimeState {
        let state = RuntimeState::new();

        state.add_process(
            JOB_ID.to_string(),
            ProcessState {
                program: "sleep".to_string(),
                args: vec!["5".to_string()],
                working_directory: None,
                environment: vec![],
                pid,
            },
        );

        state
    }

    fn create_runtime(
        kill_results: Vec<(libc::c_int, i32)>,
        wait_results: Vec<(libc::pid_t, i32)>,
        pid: Option<u32>,
    ) -> (
        StopRuntime<MockOsAdapter, MockLogger, MockMetrics>,
        Arc<Mutex<Vec<(String, f64)>>>,
        Arc<Mutex<Vec<(String, HashMap<String, String>)>>>,
        Arc<Mutex<Vec<(String, HashMap<String, String>)>>>,
    ) {
        let logger = MockLogger::new();
        let metrics = MockMetrics::new();
        let os_adapter = MockOsAdapter::new(
            kill_results,
            wait_results,
        );

        let increment_calls = metrics.increment_calls.clone();
        let info_calls = logger.info_calls.clone();
        let error_calls = logger.error_calls.clone();

        let observability = Observability {
            logger,
            metrics,
        };

        let telemetry = StopRuntimeTelemetry::new(observability);

        let runtime = StopRuntime::new(
            create_runtime_state(pid),
            telemetry,
            os_adapter,
        );

        (
            runtime,
            increment_calls,
            info_calls,
            error_calls,
        )
    }

    // ============================================================
    // 1. MAIN STOP EXECUTION PATHS
    // ============================================================

    #[test]
    fn stop_returns_success_when_process_already_exited() {
        // SIGTERM returns ESRCH: process already exited.

        let (runtime, metrics, info, _) = create_runtime(
            vec![( -1, libc::ESRCH )],
            vec![],
            Some(PID),
        );

        let result = runtime.stop(create_job());

        assert!(result.is_ok());

        assert_eq!(
            runtime.os_adapter.kill_calls.lock().unwrap().len(),
            1
        );

        assert_eq!(
            runtime.os_adapter.kill_calls.lock().unwrap()[0].1,
            libc::SIGTERM
        );

        assert_eq!(metrics.lock().unwrap()[0].0, "runtime.stop.successes");

        assert_eq!(info.lock().unwrap().len(), 1);
    }

    #[test]
    fn stop_returns_success_after_graceful_termination() {
        // SIGTERM succeeds.
        // waitpid confirms process termination.

        let (runtime, metrics, _, _) = create_runtime(
            vec![(0, 0)],
            vec![(PID as libc::pid_t, 0)],
            Some(PID),
        );

        let result = runtime.stop(create_job());

        assert!(result.is_ok());

        assert_eq!(
            runtime.os_adapter.kill_calls.lock().unwrap().len(),
            1
        );

        assert_eq!(
            runtime.os_adapter.wait_calls.lock().unwrap().len(),
            1
        );

        assert_eq!(
            metrics.lock().unwrap()[0].0,
            "runtime.stop.successes"
        );
    }

    #[test]
    fn stop_returns_success_after_forced_termination() {
        // SIGTERM succeeds.
        // First wait times out.
        // SIGKILL returns ESRCH: process is already gone.

        let (runtime, metrics, _, _) = create_runtime(
            vec![
                (0, 0),
                (-1, libc::ESRCH),
            ],
            vec![],
            Some(PID),
        );

        let result = runtime.stop(create_job());

        assert!(result.is_ok());

        let kill_calls = runtime.os_adapter.kill_calls.lock().unwrap();

        assert_eq!(kill_calls.len(), 2);
        assert_eq!(kill_calls[0].1, libc::SIGTERM);
        assert_eq!(kill_calls[1].1, libc::SIGKILL);

        assert_eq!(
            metrics.lock().unwrap()[0].0,
            "runtime.stop.successes"
        );
    }

    #[test]
    fn stop_returns_success_after_sigkill() {
        let mut wait_results = vec![(0, 0); 20];
        wait_results.push((PID as libc::pid_t, 0));

        let (runtime, metrics, _, _) = create_runtime(
            vec![(0, 0), (0, 0)],
            wait_results,
            Some(PID),
        );

        let result = runtime.stop(create_job());

        assert!(result.is_ok());

        let kill_calls = runtime.os_adapter.kill_calls.lock().unwrap();

        assert_eq!(kill_calls.len(), 2);
        assert_eq!(kill_calls[0].1, libc::SIGTERM);
        assert_eq!(kill_calls[1].1, libc::SIGKILL);

        assert_eq!(
            metrics.lock().unwrap()[0].0,
            "runtime.stop.successes"
        );
    }

    #[test]
    fn stop_returns_error_when_process_survives_sigkill() {
        // SIGTERM succeeds.
        // First wait times out.
        // SIGKILL succeeds.
        // Final wait also times out.

        let (runtime, _, _, _) = create_runtime(
            vec![
                (0, 0),
                (0, 0),
            ],
            vec![],
            Some(PID),
        );

        let result = runtime.stop(create_job());

        assert!(result.is_err());

        assert!(
            result.unwrap_err().contains("Failed to terminate process")
        );

        let kill_calls = runtime.os_adapter.kill_calls.lock().unwrap();

        assert_eq!(kill_calls.len(), 2);
        assert_eq!(kill_calls[0].1, libc::SIGTERM);
        assert_eq!(kill_calls[1].1, libc::SIGKILL);
    }

    // ============================================================
    // 2. PROCESS RESOLUTION
    // ============================================================

    #[test]
    fn stop_fails_when_job_does_not_exist() {
        let logger = MockLogger::new();
        let metrics = MockMetrics::new();

        let errors = logger.error_calls.clone();
        let increments = metrics.increment_calls.clone();

        let runtime = StopRuntime::new(
            RuntimeState::new(),
            StopRuntimeTelemetry::new(Observability {
                logger,
                metrics,
            }),
            MockOsAdapter::new(vec![], vec![]),
        );

        let result = runtime.stop(create_job());

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("No process found"));

        assert_eq!(errors.lock().unwrap().len(), 1);

        assert_eq!(
            increments.lock().unwrap()[0].0,
            "runtime.stop.failures"
        );

        assert!(runtime.os_adapter.kill_calls.lock().unwrap().is_empty());
    }

    #[test]
    fn stop_fails_when_process_has_no_pid() {
        let logger = MockLogger::new();
        let metrics = MockMetrics::new();

        let errors = logger.error_calls.clone();

        let runtime = StopRuntime::new(
            create_runtime_state(None),
            StopRuntimeTelemetry::new(Observability {
                logger,
                metrics,
            }),
            MockOsAdapter::new(vec![], vec![]),
        );

        let result = runtime.stop(create_job());

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("No PID found"));

        assert_eq!(errors.lock().unwrap().len(), 1);

        assert!(runtime.os_adapter.kill_calls.lock().unwrap().is_empty());
    }

    #[test]
    fn stop_fails_when_pid_is_invalid() {
        // u32::MAX cannot fit into a signed 32-bit libc::pid_t.

        let logger = MockLogger::new();
        let metrics = MockMetrics::new();

        let errors = logger.error_calls.clone();

        let runtime = StopRuntime::new(
            create_runtime_state(Some(u32::MAX)),
            StopRuntimeTelemetry::new(Observability {
                logger,
                metrics,
            }),
            MockOsAdapter::new(vec![], vec![]),
        );

        let result = runtime.stop(create_job());

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid PID"));

        assert_eq!(errors.lock().unwrap().len(), 1);

        assert!(runtime.os_adapter.kill_calls.lock().unwrap().is_empty());
    }

    // ============================================================
    // 3. SIGNAL HANDLING
    // ============================================================

    #[test]
    fn send_signal_returns_false_when_signal_succeeds() {
        let (runtime, _, _, _) = create_runtime(
            vec![(0, 0)],
            vec![],
            Some(PID),
        );

        let result = runtime.send_signal(
            PID as libc::pid_t,
            libc::SIGTERM,
        );

        assert_eq!(result.unwrap(), false);
    }

    #[test]
    fn send_signal_returns_true_when_process_does_not_exist() {
        let (runtime, _, _, _) = create_runtime(
            vec![(-1, libc::ESRCH)],
            vec![],
            Some(PID),
        );

        let result = runtime.send_signal(
            PID as libc::pid_t,
            libc::SIGTERM,
        );

        assert_eq!(result.unwrap(), true);
    }

    #[test]
    fn send_signal_returns_error_on_permission_denied() {
        let (runtime, _, _, _) = create_runtime(
            vec![(-1, libc::EPERM)],
            vec![],
            Some(PID),
        );

        let result = runtime.send_signal(
            PID as libc::pid_t,
            libc::SIGTERM,
        );

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Permission denied"));
    }

    #[test]
    fn send_signal_returns_error_on_other_os_error() {
        let (runtime, _, _, _) = create_runtime(
            vec![(-1, libc::EINVAL)],
            vec![],
            Some(PID),
        );

        let result = runtime.send_signal(
            PID as libc::pid_t,
            libc::SIGTERM,
        );

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Failed to signal"));
    }

    // ============================================================
    // 4. WAITING FOR PROCESS TERMINATION
    // ============================================================

    #[test]
    fn wait_returns_true_when_process_terminates() {
        let (runtime, _, _, _) = create_runtime(
            vec![],
            vec![(PID as libc::pid_t, 0)],
            Some(PID),
        );

        let result = runtime.wait_for_process_termination(
            PID as libc::pid_t,
            1,
        );

        assert_eq!(result.unwrap(), true);
    }

    #[test]
    fn wait_returns_false_when_timeout_expires() {
        let (runtime, _, _, _) = create_runtime(
            vec![],
            vec![],
            Some(PID),
        );

        let result = runtime.wait_for_process_termination(
            PID as libc::pid_t,
            0,
        );

        assert_eq!(result.unwrap(), false);

        assert!(
            runtime.os_adapter.wait_calls.lock().unwrap().is_empty()
        );
    }

    #[test]
    fn wait_retries_when_interrupted() {
        let (runtime, _, _, _) = create_runtime(
            vec![],
            vec![
                (-1, libc::EINTR),
                (PID as libc::pid_t, 0),
            ],
            Some(PID),
        );

        let result = runtime.wait_for_process_termination(
            PID as libc::pid_t,
            1,
        );

        assert_eq!(result.unwrap(), true);

        assert_eq!(
            runtime.os_adapter.wait_calls.lock().unwrap().len(),
            2
        );
    }

    #[test]
    fn wait_returns_error_on_other_waitpid_error() {
        let (runtime, _, _, _) = create_runtime(
            vec![],
            vec![(-1, libc::ECHILD)],
            Some(PID),
        );

        let result = runtime.wait_for_process_termination(
            PID as libc::pid_t,
            1,
        );

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Failed to wait"));
    }

    // ============================================================
    // 5. TELEMETRY
    // ============================================================

    #[test]
    fn successful_stop_records_success_metric_and_log() {
        let (runtime, metrics, info, _) = create_runtime(
            vec![(-1, libc::ESRCH)],
            vec![],
            Some(PID),
        );

        let result = runtime.stop(create_job());

        assert!(result.is_ok());

        let metric_calls = metrics.lock().unwrap();

        assert_eq!(metric_calls.len(), 1);
        assert_eq!(
            metric_calls[0],
            ("runtime.stop.successes".to_string(), 1.0)
        );

        let info_calls = info.lock().unwrap();

        assert_eq!(info_calls.len(), 1);

        assert_eq!(
            info_calls[0].0,
            "SIGTERM sent successfully"
        );

        assert_eq!(
            info_calls[0].1.get("job_id").unwrap(),
            JOB_ID
        );

        assert_eq!(
            info_calls[0].1.get("pid").unwrap(),
            &PID.to_string()
        );
    }

    #[test]
    fn failed_stop_records_failure_metric_and_log() {
        let logger = MockLogger::new();
        let metrics = MockMetrics::new();

        let errors = logger.error_calls.clone();
        let increments = metrics.increment_calls.clone();

        let runtime = StopRuntime::new(
            RuntimeState::new(),
            StopRuntimeTelemetry::new(Observability {
                logger,
                metrics,
            }),
            MockOsAdapter::new(vec![], vec![]),
        );

        let result = runtime.stop(create_job());

        assert!(result.is_err());

        let metric_calls = increments.lock().unwrap();

        assert_eq!(metric_calls.len(), 1);
        assert_eq!(
            metric_calls[0],
            ("runtime.stop.failures".to_string(), 1.0)
        );

        let error_calls = errors.lock().unwrap();

        assert_eq!(error_calls.len(), 1);

        assert_eq!(
            error_calls[0].0,
            "Failed to stop job"
        );

        assert_eq!(
            error_calls[0].1.get("job_id").unwrap(),
            JOB_ID
        );

        assert!(
            error_calls[0].1.contains_key("error")
        );
    }
}
