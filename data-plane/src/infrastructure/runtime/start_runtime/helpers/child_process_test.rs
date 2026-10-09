#[cfg(all(test, target_os = "linux"))]
mod tests {
    use std::{fs, time::{SystemTime, UNIX_EPOCH}};

    use crate::{
        infrastructure::runtime::start_runtime::helpers::{
            child_process::ChildProcess,
            create_child_stack::create_child_stack,
            create_process::create_process,
        },
        observability::observability::Observability,
        ports::{logger::Logger, metrics::Metrics},
    };

    use std::collections::HashMap;

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

    #[test]
    fn child_process_writes_stdout_and_stderr_to_separate_files() {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        let directory = std::env::temp_dir().join(format!(
            "child-process-test-{}-{}",
            std::process::id(),
            timestamp
        ));

        fs::create_dir_all(&directory).unwrap();

        let child = ChildProcess {
            program: "/bin/sh".to_string(),
            args: vec![
                "-c".to_string(),
                "printf 'hello stdout\\n'; printf 'hello stderr\\n' >&2".to_string(),
            ],
            working_directory: Some(directory.to_string_lossy().into_owned()),
            environment: vec![],
        };

        let observability = Observability {
            logger: TestLogger,
            metrics: TestMetrics,
        };

        let mut stack = create_child_stack();

        let result = create_process(&observability, &mut stack, child);

        let pid = match result {
            Ok(pid) => pid,
            Err(error) => {
                let _ = fs::remove_dir_all(&directory);
                panic!("Failed to create child process: {error}");
            }
        };

        let mut status: libc::c_int = 0;
        let waited = unsafe {
            libc::waitpid(pid as libc::pid_t, &mut status, 0)
        };

        assert_eq!(waited, pid as libc::pid_t);
        assert!(libc::WIFEXITED(status));
        assert_eq!(libc::WEXITSTATUS(status), 0);

        let stdout = fs::read_to_string(directory.join("stdout.txt")).unwrap();
        let stderr = fs::read_to_string(directory.join("stderr.txt")).unwrap();

        assert!(stdout.contains("hello stdout"), "stdout: {stdout:?}");
        assert!(stderr.contains("hello stderr"), "stderr: {stderr:?}");

        fs::remove_dir_all(directory).unwrap();
    }
}