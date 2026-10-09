#[cfg(test)]
mod tests {
    use std::{
        collections::HashMap,
        fs,
        path::{Path, PathBuf},
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
                    .join("gt-cloud-failure-tests")
                    .to_string_lossy()
                    .into_owned(),
            },
            observability: Observability {
                logger: TestLogger,
                metrics: TestMetrics,
            },
        }
    }

    fn create_job(job_id: &str, code_path: &Path, command: String) -> Job {
        Job {
            id: job_id.to_string(),
            app: App {
                id: format!("app-{job_id}"),
                code_path: code_path.to_string_lossy().into_owned(),
                command,
                environment: vec![],
            },
            name: format!("failure-test-{job_id}"),
            status: JobStatus::Received,
            resources: Resource {
                cpu: 1,
                memory: 128,
            },
            network: vec![],
        }
    }

    fn unique_test_dir(label: &str) -> PathBuf {
        std::env::temp_dir()
            .join("gt-cloud-failure-tests")
            .join(format!("{label}-{}", std::process::id()))
    }

    fn wait_for_stderr(path: &Path, expected_text: &str) -> String {
        let deadline = Instant::now() + Duration::from_secs(5);

        loop {
            let stderr = fs::read_to_string(path).unwrap_or_default();

            if stderr.contains(expected_text) {
                return stderr;
            }

            if Instant::now() >= deadline {
                panic!(
                    "Timed out waiting for stderr to contain {:?}. Actual stderr: {:?}",
                    expected_text, stderr
                );
            }

            thread::sleep(Duration::from_millis(20));
        }
    }

    // A configured job whose executable does not exist should not run successfully.
    // The child-process helper is expected to report the exec failure to stderr.
    #[cfg(target_os = "linux")]
    #[test]
    fn missing_executable_is_reported() {
        let state = RuntimeState::new();
        let job_id = format!("missing-executable-{}", std::process::id());
        let root = unique_test_dir("missing-executable");

        fs::create_dir_all(&root).expect("Could not create test directory");

        let nonexistent_program = "/definitely/not/a/real/gt-cloud-executable";

        let job = create_job(
            &job_id,
            &root.join("unused.txt"),
            nonexistent_program.to_string(),
        );

        let result = (|| {
            create_configure_runtime()
                .configure(&state, job.clone())
                .expect(
                    "Configuration should store the command even if the executable is absent",
                );

            let process = state
                .get_process(&job.id)
                .expect("Configured process state should exist");

            let working_directory = process
                .working_directory
                .clone()
                .expect("Working directory should be configured");

            create_runtime()
                .start(&state, job.clone())
                .expect("clone/start may succeed before the child reports exec failure");

            let stderr_path = Path::new(&working_directory).join("stderr.txt");

            let stderr =
                wait_for_stderr(&stderr_path, "No such file or directory");

            assert!(
                !stderr.is_empty(),
                "Missing executable should produce a diagnostic"
            );

            let _ = fs::remove_dir_all(&working_directory);
        })();

        let _ = fs::remove_dir_all(&root);
        result;
    }

    // Simulates a worker without Python using a Python command that does not exist.
    // This does not uninstall Python from the test host.
    #[cfg(target_os = "linux")]
    #[test]
    fn missing_python_interpreter_is_reported() {
        let state = RuntimeState::new();
        let job_id = format!("missing-python-{}", std::process::id());
        let root = unique_test_dir("missing-python");

        fs::create_dir_all(&root).expect("Could not create test directory");

        let script_path = root.join("job.py");

        fs::write(
            &script_path,
            "print('this should not run')\n",
        )
        .expect("Could not write Python script");

        let missing_python = "/definitely/not/installed/python3";

        let job = create_job(
            &job_id,
            &script_path,
            format!("{missing_python} {}", script_path.display()),
        );

        let result = (|| {
            create_configure_runtime()
                .configure(&state, job.clone())
                .expect(
                    "Configuration should not require Python to be installed",
                );

            let process = state
                .get_process(&job.id)
                .expect("Configured process state should exist");

            let working_directory = process
                .working_directory
                .clone()
                .expect("Working directory should be configured");

            create_runtime()
                .start(&state, job.clone())
                .expect(
                    "clone/start may succeed before the child reports exec failure",
                );

            let stderr_path = Path::new(&working_directory).join("stderr.txt");

            let stderr =
                wait_for_stderr(&stderr_path, "No such file or directory");

            assert!(
                !stderr.contains("this should not run"),
                "The script should not execute without an interpreter"
            );

            let _ = fs::remove_dir_all(&working_directory);
        })();

        let _ = fs::remove_dir_all(&root);
        result;
    }

    // Python exists, but the script path does not. Python should report the error.
    #[cfg(target_os = "linux")]
    #[test]
    fn missing_python_script_is_reported() {
        let python_check = std::process::Command::new("python3")
            .arg("--version")
            .output()
            .expect("python3 must be installed in the Linux test environment");

        assert!(python_check.status.success());

        let state = RuntimeState::new();
        let job_id = format!("missing-script-{}", std::process::id());
        let root = unique_test_dir("missing-script");

        fs::create_dir_all(&root).expect("Could not create test directory");

        let script_path = root.join("does-not-exist.py");

        let job = create_job(
            &job_id,
            &script_path,
            format!("python3 {}", script_path.display()),
        );

        let result = (|| {
            create_configure_runtime()
                .configure(&state, job.clone())
                .expect("Configuration should succeed before execution");

            let process = state
                .get_process(&job.id)
                .expect("Configured process state should exist");

            let working_directory = process
                .working_directory
                .clone()
                .expect("Working directory should be configured");

            create_runtime()
                .start(&state, job.clone())
                .expect(
                    "Process creation should succeed before Python reports the missing script",
                );

            let stderr_path = Path::new(&working_directory).join("stderr.txt");

            let stderr =
                wait_for_stderr(&stderr_path, "does-not-exist.py");

            let stderr_lowercase = stderr.to_lowercase();

            assert!(
                stderr_lowercase.contains("no such file")
                    || stderr_lowercase.contains("can't open file")
                    || stderr_lowercase.contains("cannot open"),
                "Expected Python to report a missing script; stderr was: {stderr}"
            );

            let _ = fs::remove_dir_all(&working_directory);
        })();

        let _ = fs::remove_dir_all(&root);
        result;
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn unconfigured_job_is_rejected() {
        let state = RuntimeState::new();

        let job = create_job(
            &format!("unconfigured-{}", std::process::id()),
            Path::new("/tmp/unused.py"),
            "python3 /tmp/unused.py".to_string(),
        );

        let error = create_runtime()
            .start(&state, job.clone())
            .expect_err("An unconfigured job must not start");

        assert_eq!(
            error,
            format!("No process state found for job {}", job.id)
        );
    }
}