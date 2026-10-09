#[cfg(all(test, target_os = "linux"))]
mod tests {
    use std::collections::HashMap;

    use crate::{
        infrastructure::runtime::start_runtime::helpers::{
            child_process::ChildProcess,
            create_child_stack::create_child_stack,
            create_process::create_process,
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

    #[test]
    fn creates_child_process_and_records_pid() {
        let observability = Observability {
            logger: TestLogger,
            metrics: TestMetrics,
        };

        let child = ChildProcess {
            program: "/bin/true".into(),
            args: vec![],
            working_directory: Some("/tmp".into()),
            environment: vec![],
        };

        let mut stack = create_child_stack();

        let pid = create_process(&observability, &mut stack, child)
            .expect("Failed to create child process");

        assert!(pid > 0);

        let mut status: libc::c_int = 0;
        let waited = unsafe {
            libc::waitpid(pid as libc::pid_t, &mut status, 0)
        };

        assert_eq!(waited, pid as libc::pid_t);
        assert!(libc::WIFEXITED(status));
        assert_eq!(libc::WEXITSTATUS(status), 0);
    }
}