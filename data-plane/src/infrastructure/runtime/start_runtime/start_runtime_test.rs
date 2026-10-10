
#[cfg(test)]
mod tests {
    use std::{
        collections::HashMap,
        fs,
        path::Path,
        process::Command,
        thread,
        time::{Duration, Instant},
    };

    use crate::{
        domain::{
            app::app::App,
            job::{job::Job, job_status::JobStatus},
            resource::resource::Resource,
        },
        infrastructure::runtime::{
            configure_runtime::configure_runtime::ConfigureRuntime,
            helpers::working_directory_manager::WorkingDirectoryManager,
            start_runtime::start_runtime::StartRuntime,
            states::runtime_state::RuntimeState,
        },
        observability::observability::Observability,
        ports::{logger::Logger, metrics::Metrics},
    };

    struct TestLogger;

    impl Logger for TestLogger {
        fn info(&self, _: &str, _: HashMap<String, String>) {}
        fn error(&self, _: &str, _: HashMap<String, String>) {}
    }

    struct TestMetrics;

    impl Metrics for TestMetrics {
        fn increment(&self, _: &str, _: f64) {}
        fn observe(&self, _: &str, _: f64) {}
    }

    fn create_job() -> Job {
        Job {
            id: format!("job-test-{}", std::process::id()),
            app: App {
                id: "app-test".to_string(),
                code_path: "/tmp/app".to_string(),
                command: "/bin/sleep 5".to_string(),
                environment: vec![],
            },
            name: "runtime-test".to_string(),
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

    fn create_configure_runtime() -> ConfigureRuntime<TestLogger, TestMetrics> {
        ConfigureRuntime {
            working_directory_manager: WorkingDirectoryManager {
                root: std::env::temp_dir()
                    .join("gt-cloud-tests")
                    .to_string_lossy()
                    .into_owned(),
            },
            observability: Observability {
                logger: TestLogger,
                metrics: TestMetrics,
            },
        }
    }

    fn kill_process(pid: u32) {
        let _ = Command::new("kill")
            .arg("-KILL")
            .arg(pid.to_string())
            .status();
    }

    #[cfg(not(target_os = "linux"))]
    #[test]
    fn start_fails_on_non_linux() {
        let runtime = create_runtime();
        let state = RuntimeState::new();

        let error = runtime
            .start(&state, create_job())
            .expect_err("Runtime should reject non-Linux platforms");

        assert_eq!(
            error,
            "Runtime process isolation is only supported on Linux"
        );
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn configure_runtime_stores_executable_and_arguments() {
        let state = RuntimeState::new();
        let job = create_job();

        create_configure_runtime()
            .configure(&state, job.clone())
            .expect("Runtime configuration should succeed");

        let process = state
            .get_process(&job.id)
            .expect("Process state should exist");

        assert_eq!(process.program, "/bin/sleep");
        assert_eq!(process.args, vec!["5".to_string()]);
        assert!(
            process
                .working_directory
                .as_ref()
                .is_some_and(|directory| Path::new(directory).is_dir()),
            "Configured working directory should exist"
        );
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn starts_job_and_executes_requested_program() {
        let state = RuntimeState::new();
        let job = create_job();

        create_configure_runtime()
            .configure(&state, job.clone())
            .expect("Configuration failed");

        let process = state
            .get_process(&job.id)
            .expect("Process state missing");

        assert_eq!(process.program, "/bin/sleep");
        assert_eq!(process.args, vec!["5".to_string()]);

        create_runtime()
            .start(&state, job.clone())
            .expect("StartRuntime::start() failed");

        let pid = state
            .get_process(&job.id)
            .expect("Process state missing after start")
            .pid
            .expect("Runtime did not record a PID");

        assert!(pid > 0);

        let result = (|| {
            let deadline = Instant::now() + Duration::from_secs(2);

            loop {
                if let Ok(executable) = fs::read_link(format!("/proc/{}/exe", pid)) {
                    if executable.file_name().and_then(|name| name.to_str())
                        == Some("sleep")
                    {
                        break;
                    }

                    if Instant::now() >= deadline {
                        panic!(
                            "Expected /bin/sleep, but PID {} executes {:?}",
                            pid, executable
                        );
                    }
                } else if Instant::now() >= deadline {
                    panic!(
                        "Could not inspect /proc/{}/exe; child may have exited",
                        pid
                    );
                }

                thread::sleep(Duration::from_millis(10));
            }

            let status = Command::new("kill")
                .arg("-0")
                .arg(pid.to_string())
                .status()
                .expect("Failed to check child PID");

            assert!(status.success(), "Sleep process should still be running");
        })();

        kill_process(pid);
        result;
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn starts_job_in_different_pid_namespace() {
        let state = RuntimeState::new();
        let job = create_job();

        create_configure_runtime()
            .configure(&state, job.clone())
            .expect("Configuration failed");

        create_runtime()
            .start(&state, job.clone())
            .expect("Runtime failed to start");

        let pid = state
            .get_process(&job.id)
            .expect("Process state missing")
            .pid
            .expect("PID missing");

        let result = (|| {
            let host_namespace = fs::read_link("/proc/self/ns/pid")
                .expect("Could not read host PID namespace");

            let child_namespace = fs::read_link(format!("/proc/{}/ns/pid", pid))
                .expect("Could not read child PID namespace");

            assert_ne!(
                host_namespace, child_namespace,
                "Job should use a different PID namespace"
            );
        })();

        kill_process(pid);
        result;
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn start_fails_when_job_is_not_configured() {
        let state = RuntimeState::new();
        let job = create_job();
        let expected = format!("No process state found for job {}", job.id);

        let error = create_runtime()
            .start(&state, job)
            .expect_err("Starting an unconfigured job should fail");

        assert_eq!(error, expected);
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn executes_python_script_and_captures_output() {
        let python_check = Command::new("python3")
            .arg("--version")
            .output()
            .expect("python3 must be installed in the test environment");

        assert!(python_check.status.success());

        let state = RuntimeState::new();
        let job_id = format!("python-test-{}", std::process::id());

        // Create the working directory and Python script.
        let root = std::env::temp_dir()
            .join("gt-cloud-tests")
            .join(&job_id);

        fs::create_dir_all(&root).expect("Failed to create Python test directory");

        let script_path = root.join("test_job.py");

        fs::write(
            &script_path,
            "print('Hello from GT Cloud!')\nprint(2 + 3)\n",
        )
        .expect("Failed to write Python script");

        let job = Job {
            id: job_id.clone(),
            app: App {
                id: "python-app-test".to_string(),
                code_path: script_path.to_string_lossy().into_owned(),
                command: format!("python3 {}", script_path.display()),
                environment: vec![],
            },
            name: "python-runtime-test".to_string(),
            status: JobStatus::Received,
            resources: Resource {
                cpu: 1,
                memory: 128,
            },
            network: vec![],
        };

        let configure = create_configure_runtime();
        let runtime = create_runtime();

        let result = (|| {
            configure
                .configure(&state, job.clone())
                .expect("Python runtime configuration failed");

            let process = state
                .get_process(&job.id)
                .expect("Python process state missing");

            let working_directory = process
                .working_directory
                .clone()
                .expect("Python working directory missing");

            runtime
                .start(&state, job.clone())
                .expect("Failed to start Python job");

            let stdout_path = Path::new(&working_directory).join("stdout.txt");
            let stderr_path = Path::new(&working_directory).join("stderr.txt");

            let deadline = Instant::now() + Duration::from_secs(5);

            loop {
                let stdout = fs::read_to_string(&stdout_path).unwrap_or_default();
                let stderr = fs::read_to_string(&stderr_path).unwrap_or_default();

                if stdout == "Hello from GT Cloud!\n5\n" {
                    assert!(
                        stderr.is_empty(),
                        "Expected empty stderr, got: {stderr}"
                    );
                    break;
                }

                if Instant::now() >= deadline {
                    panic!(
                        "Python output mismatch.\nExpected: {:?}\nActual stdout: {:?}\nActual stderr: {:?}",
                        "Hello from GT Cloud!\n5\n",
                        stdout,
                        stderr
                    );
                }

                thread::sleep(Duration::from_millis(20));
            }

            let _ = fs::remove_dir_all(&working_directory);
        })();

        let _ = fs::remove_dir_all(&root);

        result;
    }


}

