
#[cfg(all(test, target_os = "linux"))]
mod tests {
    use crate::infrastructure::runtime::start_runtime::helpers::{
        child_process::{
            append_exec_error,
            ChildProcess,
        },
        process_executor::ProcessExecutor,
    };

    use std::{
        fs,
        io::{self, Write},
        path::PathBuf,
        process::Command as TestCommand,
        sync::atomic::{AtomicU64, Ordering},
    };

    static TEST_ID: AtomicU64 = AtomicU64::new(0);

    fn temp_dir(name: &str) -> PathBuf {
        let id = TEST_ID.fetch_add(1, Ordering::Relaxed);

        let path = std::env::temp_dir().join(format!(
            "child_process_{}_{}_{}",
            name,
            std::process::id(),
            id
        ));

        fs::create_dir_all(&path).unwrap();
        path
    }

    fn context(
        program: &str,
        directory: Option<String>,
    ) -> Box<ChildProcess> {
        Box::new(ChildProcess {
            program: program.to_string(),
            args: vec!["--invalid-argument".to_string()],
            working_directory: directory,
            environment: vec![
                ("TEST_ENV".to_string(), "test_value".to_string()),
            ],
        })
    }

    fn run_context(context: Box<ChildProcess>) -> libc::c_int {
        ChildProcess::run(Box::into_raw(context) as *mut libc::c_void)
    }

    fn run_context_with_mock(
        context: Box<ChildProcess>,
        executor: &dyn ProcessExecutor,
    ) -> libc::c_int {
        ChildProcess::run_with_executor(context, executor)
    }

    // ---------------------------------------------------------
    // Mock executor
    // ---------------------------------------------------------

    struct MockProcessExecutor {
        should_fail: bool,
    }

    impl ProcessExecutor for MockProcessExecutor {
        fn execute(
            &self,
            _program: &str,
            _args: &[String],
            _working_directory: Option<&str>,
            _environment: &[(String, String)],
            mut stdout: fs::File,
            mut stderr: fs::File,
        ) -> io::Result<()> {
            if self.should_fail {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    "mock execution failure",
                ));
            }

            writeln!(stdout, "mock stdout")?;
            writeln!(stderr, "mock stderr")?;

            Ok(())
        }
    }

    // ---------------------------------------------------------
    // Null context
    // ---------------------------------------------------------

    #[test]
    fn null_context_returns_127() {
        let result = ChildProcess::run(std::ptr::null_mut());

        assert_eq!(result, 127);
    }

    // ---------------------------------------------------------
    // Log creation failures
    // ---------------------------------------------------------

    #[test]
    fn stdout_creation_failure_returns_127() {
        let root = temp_dir("stdout_failure");
        let missing = root.join("missing");

        let result = run_context(context(
            "/bin/sh",
            Some(missing.to_string_lossy().into_owned()),
        ));

        assert_eq!(result, 127);

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn stderr_creation_failure_returns_127() {
        let root = temp_dir("stderr_failure");

        fs::create_dir(root.join("stderr.txt")).unwrap();

        let result = run_context(context(
            "/bin/sh",
            Some(root.to_string_lossy().into_owned()),
        ));

        assert_eq!(result, 127);

        assert!(root.join("stdout.txt").exists());

        fs::remove_dir_all(root).unwrap();
    }

    // ---------------------------------------------------------
    // Mock execution tests
    // ---------------------------------------------------------

    #[test]
    fn successful_mock_execution_returns_zero() {
        let root = temp_dir("mock_success");

        let executor = MockProcessExecutor {
            should_fail: false,
        };

        let result = run_context_with_mock(
            context(
                "test_program",
                Some(root.to_string_lossy().into_owned()),
            ),
            &executor,
        );

        assert_eq!(result, 0);

        let stdout = fs::read_to_string(root.join("stdout.txt")).unwrap();
        let stderr = fs::read_to_string(root.join("stderr.txt")).unwrap();

        assert_eq!(stdout, "mock stdout\n");
        assert_eq!(stderr, "mock stderr\n");

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn failed_mock_execution_returns_127_and_logs_error() {
        let root = temp_dir("mock_failure");

        let executor = MockProcessExecutor {
            should_fail: true,
        };

        let result = run_context_with_mock(
            context(
                "test_program",
                Some(root.to_string_lossy().into_owned()),
            ),
            &executor,
        );

        assert_eq!(result, 127);

        let stderr = fs::read_to_string(root.join("stderr.txt")).unwrap();

        assert!(stderr.contains("Failed to execute"));
        assert!(stderr.contains("test_program"));
        assert!(stderr.contains("mock execution failure"));

        fs::remove_dir_all(root).unwrap();
    }

    // ---------------------------------------------------------
    // Error logging
    // ---------------------------------------------------------

    #[test]
    fn append_exec_error_handles_missing_log_file() {
        let root = temp_dir("append_failure");
        let missing_log = root.join("missing.txt");

        let error = io::Error::new(
            io::ErrorKind::NotFound,
            "test execution error",
        );

        append_exec_error(
            &missing_log,
            "test_program",
            &[],
            &error,
        );

        assert!(!missing_log.exists());

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn append_exec_error_writes_to_existing_log() {
        let root = temp_dir("append_success");
        let stderr_path = root.join("stderr.txt");

        fs::write(&stderr_path, "Initial log\n").unwrap();

        let error = io::Error::new(
            io::ErrorKind::NotFound,
            "execution failed",
        );

        append_exec_error(
            &stderr_path,
            "test_program",
            &["argument".to_string()],
            &error,
        );

        let contents = fs::read_to_string(&stderr_path).unwrap();

        assert!(contents.contains("Initial log"));
        assert!(contents.contains("Failed to execute"));
        assert!(contents.contains("test_program"));

        fs::remove_dir_all(root).unwrap();
    }

    // ---------------------------------------------------------
    // Real subprocess integration test
    // ---------------------------------------------------------

    // This helper runs only in the child test process.
    // A successful exec replaces this process with /bin/sh.
    #[test]
    fn exec_success_helper() {
        if std::env::var("CHILD_PROCESS_EXEC_HELPER").ok().as_deref()
            != Some("1")
        {
            return;
        }

        let child = Box::new(ChildProcess {
            program: "/bin/sh".to_string(),
            args: vec![
                "-c".to_string(),
                "printf '%s' \"$TEST_ENV\"; printf 'stderr-output' >&2"
                    .to_string(),
            ],
            working_directory: None,
            environment: vec![
                (
                    "TEST_ENV".to_string(),
                    "environment-success".to_string(),
                ),
            ],
        });

        let result = run_context(child);

        std::process::exit(result);
    }

    #[test]
    fn successful_exec_creates_stdout_and_stderr_logs() {
        let root = temp_dir("exec_success");

        let executable = std::env::current_exe().unwrap();

        let status = TestCommand::new(executable)
            .arg("exec_success_helper")
            .env_clear()
            .env("CHILD_PROCESS_EXEC_HELPER", "1")
            .current_dir(&root)
            .status()
            .unwrap();

        assert!(
            status.success(),
            "Child process failed with status: {:?}",
            status
        );

        let stdout = fs::read_to_string(root.join("stdout.txt")).unwrap();
        let stderr = fs::read_to_string(root.join("stderr.txt")).unwrap();

        assert_eq!(stdout, "environment-success");
        assert_eq!(stderr, "stderr-output");

        fs::remove_dir_all(root).unwrap();
    }
}
