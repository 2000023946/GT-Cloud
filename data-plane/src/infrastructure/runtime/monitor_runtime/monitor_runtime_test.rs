
#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        collections::HashMap,
        sync::{Arc, Mutex},
    };

    use crate::{
        domain::{
            app::App,
            job::{Job, JobStatus},
            network::NetworkRule,
            resource::Resource,
            runtime_states::runtime_states::RuntimeStates,
        },
        infrastructure::runtime::{
            monitor_runtime::monitor_runtime::Monitor,
            states::runtime_state::RuntimeState,
        },
        observability::observability::Observability,
    };

    use crate::infrastructure::runtime::states::process_state::ProcessState;
    use crate::ports::{logger::Logger, metrics::Metrics};

    struct TestLogger {
        messages: Arc<Mutex<Vec<String>>>,
    }

    impl Logger for TestLogger {
        fn info(&self, message: &str, _: HashMap<String, String>) {
            self.messages.lock().unwrap().push(message.to_string());
        }

        fn error(&self, message: &str, _: HashMap<String, String>) {
            self.messages.lock().unwrap().push(message.to_string());
        }
    }

    struct TestMetrics {
        increments: Arc<Mutex<Vec<(String, f64)>>>,
    }

    impl Metrics for TestMetrics {
        fn increment(&self, name: &str, value: f64) {
            self.increments
                .lock()
                .unwrap()
                .push((name.to_string(), value));
        }

        fn observe(&self, _: &str, _: f64) {}
    }

    static mut MOCK_RESULT: libc::pid_t = 0;
    static mut MOCK_STATUS: libc::c_int = 0;

    fn mock_waitpid(
        _: libc::pid_t,
        status: *mut libc::c_int,
        _: libc::c_int,
    ) -> libc::pid_t {
        unsafe {
            *status = MOCK_STATUS;
            MOCK_RESULT
        }
    }

    fn configure_mock(result: libc::pid_t, status: libc::c_int) {
        unsafe {
            MOCK_RESULT = result;
            MOCK_STATUS = status;
        }
    }

    fn make_job(id: &str) -> Job {
        Job {
            id: id.to_string(),
            app: App {
                id: "app-1".to_string(),
                code_path: "/tmp/app".to_string(),
                command: "test".to_string(),
                environment: vec![],
            },
            name: "test-job".to_string(),
            status: JobStatus::Running,
            resources: Resource {
                cpu: 1,
                memory: 1024,
            },
            network: Vec::<NetworkRule>::new(),
        }
    }

    fn make_process(pid: Option<u32>) -> ProcessState {
        ProcessState {
            program: "test".to_string(),
            args: vec![],
            working_directory: None,
            environment: vec![],
            pid,
        }
    }

    fn make_logger() -> TestLogger {
        TestLogger {
            messages: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn make_metrics() -> TestMetrics {
        TestMetrics {
            increments: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn make_monitor(
        state: RuntimeState,
        logger: TestLogger,
        metrics: TestMetrics,
    ) -> Monitor<TestLogger, TestMetrics> {
        Monitor::with_waitpid(
            state,
            Observability { logger, metrics },
            mock_waitpid,
        )
    }

    #[test]
    fn missing_process_returns_not_found() {
        let monitor =
            make_monitor(RuntimeState::new(), make_logger(), make_metrics());

        let result = monitor.monitor(make_job("missing"));

        assert!(matches!(result, Ok(RuntimeStates::NotFound)));
    }

    #[test]
    fn missing_pid_returns_not_found() {
        let state = RuntimeState::new();
        state.add_process("job-1".to_string(), make_process(None));

        let monitor = make_monitor(state, make_logger(), make_metrics());
        let result = monitor.monitor(make_job("job-1"));

        assert!(matches!(result, Ok(RuntimeStates::NotFound)));
    }

    #[test]
    fn zero_waitpid_result_returns_running() {
        configure_mock(0, 0);

        let state = RuntimeState::new();
        state.add_process("job-1".to_string(), make_process(Some(1234)));

        let monitor = make_monitor(state, make_logger(), make_metrics());
        let result = monitor.monitor(make_job("job-1"));

        assert!(matches!(result, Ok(RuntimeStates::Running)));
        assert!(monitor.runtime_state.get_status("job-1").is_none());
    }

    #[test]
    fn exited_process_returns_and_saves_exit_code() {
        configure_mock(1234, 7 << 8);

        let state = RuntimeState::new();
        state.add_process("job-1".to_string(), make_process(Some(1234)));

        let monitor = make_monitor(state, make_logger(), make_metrics());
        let result = monitor.monitor(make_job("job-1"));

        assert!(matches!(
            result,
            Ok(RuntimeStates::Exited { code: 7 })
        ));

        assert!(matches!(
            monitor.runtime_state.get_status("job-1"),
            Some(RuntimeStates::Exited { code: 7 })
        ));
    }

    #[test]
    fn signaled_process_returns_and_saves_signal() {
        configure_mock(1234, libc::SIGTERM);

        let state = RuntimeState::new();
        state.add_process("job-1".to_string(), make_process(Some(1234)));

        let monitor = make_monitor(state, make_logger(), make_metrics());
        let result = monitor.monitor(make_job("job-1"));

        assert!(matches!(
            result,
            Ok(RuntimeStates::Signaled { signal })
                if signal == libc::SIGTERM
        ));

        assert!(matches!(
            monitor.runtime_state.get_status("job-1"),
            Some(RuntimeStates::Signaled { signal })
                if signal == libc::SIGTERM
        ));
    }

    #[test]
    fn waitpid_error_returns_error_state() {
        configure_mock(-1, 0);

        let state = RuntimeState::new();
        state.add_process("job-1".to_string(), make_process(Some(1234)));

        let monitor = make_monitor(state, make_logger(), make_metrics());
        let result = monitor.monitor(make_job("job-1"));

        assert!(matches!(result, Ok(RuntimeStates::Error)));
    }

    #[test]
    fn unexpected_waitpid_result_returns_error() {
        configure_mock(5678, 0);

        let state = RuntimeState::new();
        state.add_process("job-1".to_string(), make_process(Some(1234)));

        let monitor = make_monitor(state, make_logger(), make_metrics());
        let result = monitor.monitor(make_job("job-1"));

        assert!(result.is_err());
    }

    #[test]
    fn previously_saved_exit_status_is_returned_without_waitpid() {
        let state = RuntimeState::new();
        state.save_status(
            "job-1".to_string(),
            RuntimeStates::Exited { code: 3 },
        );

        let monitor = make_monitor(state, make_logger(), make_metrics());
        let result = monitor.monitor(make_job("job-1"));

        assert!(matches!(
            result,
            Ok(RuntimeStates::Exited { code: 3 })
        ));
    }

    #[test]
    fn previously_saved_signal_status_is_returned_without_waitpid() {
        let state = RuntimeState::new();
        state.save_status(
            "job-1".to_string(),
            RuntimeStates::Signaled {
                signal: libc::SIGTERM,
            },
        );

        let monitor = make_monitor(state, make_logger(), make_metrics());
        let result = monitor.monitor(make_job("job-1"));

        assert!(matches!(
            result,
            Ok(RuntimeStates::Signaled { signal })
                if signal == libc::SIGTERM
        ));
    }

    #[test]
    fn saved_running_status_does_not_short_circuit_monitoring() {
        configure_mock(0, 0);

        let state = RuntimeState::new();
        state.add_process("job-1".to_string(), make_process(Some(1234)));
        state.save_status("job-1".to_string(), RuntimeStates::Running);

        let monitor = make_monitor(state, make_logger(), make_metrics());
        let result = monitor.monitor(make_job("job-1"));

        assert!(matches!(result, Ok(RuntimeStates::Running)));
    }

    #[test]
    fn monitoring_logs_job() {
        let state = RuntimeState::new();
        let logger = make_logger();
        let messages = Arc::clone(&logger.messages);

        let monitor = make_monitor(state, logger, make_metrics());

        let _ = monitor.monitor(make_job("job-1"));

        assert!(
            messages
                .lock()
                .unwrap()
                .iter()
                .any(|message| message == "Monitoring job")
        );
    }

    #[test]
    fn monitoring_increments_metric() {
        let state = RuntimeState::new();
        let metrics = make_metrics();
        let increments = Arc::clone(&metrics.increments);

        let monitor = make_monitor(state, make_logger(), metrics);

        let _ = monitor.monitor(make_job("job-1"));

        assert!(
            increments
                .lock()
                .unwrap()
                .iter()
                .any(|(name, value)| {
                    name == "monitor.checks" && *value == 1.0
                })
        );
    }

    #[test]
    fn saved_error_status_does_not_skip_monitoring() {
        configure_mock(0, 0);

        let state = RuntimeState::new();
        state.add_process("job-1".to_string(), make_process(Some(1234)));
        state.save_status("job-1".to_string(), RuntimeStates::Error);

        let monitor = make_monitor(state, make_logger(), make_metrics());

        let result = monitor.monitor(make_job("job-1"));

        // Error is not a terminal status, so monitoring continues.
        assert!(matches!(result, Ok(RuntimeStates::Running)));

        // Running does not overwrite the previously saved Error status.
        assert!(matches!(
            monitor.runtime_state.get_status("job-1"),
            Some(RuntimeStates::Error)
        ));
    }

    #[test]
    fn stopped_process_status_returns_unexpected_status_error() {
        // A stopped-process wait status is neither exited nor signaled.
        configure_mock(1234, 0x7f);

        let state = RuntimeState::new();
        state.add_process("job-1".to_string(), make_process(Some(1234)));

        let monitor = make_monitor(state, make_logger(), make_metrics());

        let result = monitor.monitor(make_job("job-1"));

        assert!(result.is_err());
    }

    #[test]
    fn real_waitpid_error_returns_error_state() {
        let state = RuntimeState::new();

        // The current process is not a child of itself, so waitpid
        // should return -1 with ECHILD.
        let pid = std::process::id();
        state.add_process(
            "job-1".to_string(),
            make_process(Some(pid)),
        );

        let monitor = Monitor::new(
            state,
            Observability {
                logger: make_logger(),
                metrics: make_metrics(),
            },
        );

        let result = monitor.monitor(make_job("job-1"));

        assert!(matches!(result, Ok(RuntimeStates::Error)));
    }
}
