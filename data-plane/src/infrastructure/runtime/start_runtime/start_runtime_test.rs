#[cfg(test)]
mod tests {
    use std::{
        collections::HashMap,
        fs,
        process::Command,
    };

    use crate::{
        domain::{
            app::app::App,
            job::{
                job::Job,
                job_status::JobStatus,
            },
            resource::resource::Resource,
        },
        infrastructure::runtime::{
            configure_runtime::configure_runtime::ConfigureRuntime,
            helpers::working_directory_manager::WorkingDirectoryManager,
            start_runtime::start_runtime::StartRuntime,
            states::runtime_state::RuntimeState,
        },
        observability::observability::Observability,
        ports::{
            logger::Logger,
            metrics::Metrics,
        },
    };

    struct TestLogger;

    impl Logger for TestLogger {
        fn info(
            &self,
            _message: &str,
            _fields: HashMap<String, String>,
        ) {
        }

        fn error(
            &self,
            _message: &str,
            _fields: HashMap<String, String>,
        ) {
        }
    }

    struct TestMetrics;

    impl Metrics for TestMetrics {
        fn increment(
            &self,
            _name: &str,
            _value: f64,
        ) {
        }

        fn observe(
            &self,
            _name: &str,
            _value: f64,
        ) {
        }
    }

    fn create_job() -> Job {
        Job {
            id: "job-1".to_string(),

            app: App {
                id: "app-1".to_string(),
                code_path: "/tmp/app".to_string(),
                command: "sleep 5".to_string(),
                environment: vec![],
            },

            name: "test-job".to_string(),

            status: JobStatus::Received,

            resources: Resource {
                cpu: 1,
                memory: 128,
            },

            network: vec![],
        }
    }

    fn create_runtime() -> StartRuntime<TestLogger, TestMetrics> {
        StartRuntime {
            observability: Observability {
                logger: TestLogger,
                metrics: TestMetrics,
            },
        }
    }

    fn create_configure_runtime()
        -> ConfigureRuntime<TestLogger, TestMetrics>
    {
        ConfigureRuntime {
            working_directory_manager:
                WorkingDirectoryManager {
                    root: "/tmp/gt-cloud-tests".to_string(),
                },

            observability: Observability {
                logger: TestLogger,
                metrics: TestMetrics,
            },
        }
    }

    #[cfg(not(target_os = "linux"))]
    #[test]
    fn start_fails_on_non_linux() {
        let runtime = create_runtime();

        let state = RuntimeState::new();

        let job = create_job();

        let result = runtime.start(
            &state,
            job,
        );

        assert!(result.is_err());

        assert_eq!(
            result.unwrap_err(),
            "Runtime process isolation is only supported on Linux"
        );
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn starts_valid_job() {
        let state = RuntimeState::new();

        let job = create_job();

        let configure_runtime =
            create_configure_runtime();

        configure_runtime
            .configure(
                &state,
                job.clone(),
            )
            .unwrap();

        let runtime = create_runtime();

        let result = runtime.start(
            &state,
            job.clone(),
        );

        assert!(
            result.is_ok(),
            "Runtime failed to start job: {:?}",
            result
        );

        let process_state = state
            .get_process(&job.id)
            .unwrap();

        assert!(
            process_state.pid.is_some(),
            "Process PID was not recorded"
        );

        let pid = process_state
            .pid
            .unwrap();

        assert!(
            pid > 0,
            "PID should be greater than zero"
        );

        let status = Command::new("kill")
            .arg("-0")
            .arg(pid.to_string())
            .status()
            .unwrap();

        assert!(
            status.success(),
            "Started process does not exist"
        );

        let _ = Command::new("kill")
            .arg(pid.to_string())
            .status();
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn starts_job_in_different_pid_namespace() {
        let state = RuntimeState::new();

        let job = create_job();

        let configure_runtime =
            create_configure_runtime();

        configure_runtime
            .configure(
                &state,
                job.clone(),
            )
            .unwrap();

        let runtime = create_runtime();

        runtime
            .start(
                &state,
                job.clone(),
            )
            .unwrap();

        let pid = state
            .get_process(&job.id)
            .unwrap()
            .pid
            .unwrap();

        let host_namespace =
            fs::read_link("/proc/self/ns/pid")
                .unwrap();

        let child_namespace =
            fs::read_link(
                format!(
                    "/proc/{}/ns/pid",
                    pid
                ),
            )
            .unwrap();

        assert_ne!(
            host_namespace,
            child_namespace,
            "Job should run in a different PID namespace"
        );

        let _ = Command::new("kill")
            .arg(pid.to_string())
            .status();
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn start_fails_when_job_is_not_configured() {
        let state = RuntimeState::new();

        let job = create_job();

        let runtime = create_runtime();

        let result = runtime.start(
            &state,
            job,
        );

        assert!(result.is_err());

        assert_eq!(
            result.unwrap_err(),
            "No process state found for job job-1"
        );
    }
}