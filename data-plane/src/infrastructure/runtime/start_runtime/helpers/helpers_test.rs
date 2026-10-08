
#[cfg(all(test, target_os = "linux"))]
mod tests {
    use crate::infrastructure::runtime::states::process_state::ProcessState;
    use crate::infrastructure::runtime::start_runtime::helpers::child_process::ChildProcess;
    use crate::infrastructure::runtime::start_runtime::helpers::create_child_process::create_child_process;
    use crate::infrastructure::runtime::start_runtime::helpers::create_child_stack::create_child_stack;

    fn make_process_state() -> ProcessState {
        ProcessState {
            program: "/bin/echo".to_string(),
            args: vec!["hello".to_string()],
            working_directory: Some("/tmp".to_string()),
            environment: vec![("MODE".to_string(), "test".to_string())],
            pid: None,
        }
    }

    fn make_child(
        program: &str,
        working_directory: Option<&str>,
        args: Vec<&str>,
        environment: Vec<(&str, &str)>,
    ) -> ChildProcess {
        ChildProcess {
            program: program.to_string(),
            args: args.into_iter().map(str::to_string).collect(),
            working_directory: working_directory.map(str::to_string),
            environment: environment
                .into_iter()
                .map(|(key, value)| (key.to_string(), value.to_string()))
                .collect(),
        }
    }

    // create_child_process tests

    #[test]
    fn create_child_process_copies_all_fields() {
        let child = create_child_process(make_process_state());

        assert_eq!(child.program, "/bin/echo");
        assert_eq!(child.args, vec!["hello"]);
        assert_eq!(child.working_directory.as_deref(), Some("/tmp"));
        assert_eq!(
            child.environment,
            vec![("MODE".to_string(), "test".to_string())]
        );
    }

    #[test]
    fn create_child_process_handles_empty_optional_fields() {
        let state = ProcessState {
            program: "/bin/true".to_string(),
            args: vec![],
            working_directory: None,
            environment: vec![],
            pid: None,
        };

        let child = create_child_process(state);

        assert_eq!(child.program, "/bin/true");
        assert!(child.args.is_empty());
        assert!(child.working_directory.is_none());
        assert!(child.environment.is_empty());
    }

    #[test]
    fn create_child_process_preserves_multiple_arguments_and_environment() {
        let state = ProcessState {
            program: "/bin/echo".to_string(),
            args: vec!["one".to_string(), "two words".to_string()],
            working_directory: Some("/".to_string()),
            environment: vec![
                ("FIRST".to_string(), "1".to_string()),
                ("SECOND".to_string(), "two".to_string()),
            ],
            pid: Some(1234),
        };

        let child = create_child_process(state);

        assert_eq!(child.args, vec!["one", "two words"]);
        assert_eq!(child.working_directory.as_deref(), Some("/"));
        assert_eq!(
            child.environment,
            vec![
                ("FIRST".to_string(), "1".to_string()),
                ("SECOND".to_string(), "two".to_string()),
            ]
        );
    }

    // create_child_stack tests

    #[test]
    fn create_child_stack_has_expected_size() {
        assert_eq!(create_child_stack().len(), 1024 * 1024);
    }

    #[test]
    fn create_child_stack_is_zero_initialized() {
        let stack = create_child_stack();

        assert!(stack.iter().all(|byte| *byte == 0));
    }

    #[test]
    fn create_child_stack_returns_independent_allocations() {
        let mut first = create_child_stack();
        let second = create_child_stack();

        first[0] = 42;

        assert_eq!(first[0], 42);
        assert_eq!(second[0], 0);
        assert_eq!(first.len(), second.len());
    }

    // ChildProcess::run failure-path tests.
    // Successful exec() replaces the current process, so do not
    // call it directly with a valid executable in the test runner.

    #[test]
    fn child_process_run_returns_one_for_missing_program() {
        let context = Box::new(make_child(
            "/definitely/not/a/real/program",
            None,
            vec![],
            vec![],
        ));

        let result =
            ChildProcess::run(Box::into_raw(context) as *mut libc::c_void);

        assert_eq!(result, 1);
    }

    #[test]
    fn child_process_run_returns_one_for_invalid_working_directory() {
        let context = Box::new(make_child(
            "/bin/echo",
            Some("/definitely/not/a/real/directory"),
            vec!["hello"],
            vec![],
        ));

        let result =
            ChildProcess::run(Box::into_raw(context) as *mut libc::c_void);

        assert_eq!(result, 1);
    }

    #[test]
    fn child_process_run_handles_arguments_and_environment_before_exec_failure() {
        let context = Box::new(make_child(
            "/definitely/not/a/real/program",
            Some("/tmp"),
            vec!["first", "second argument"],
            vec![("MODE", "test"), ("EMPTY", "")],
        ));

        let result =
            ChildProcess::run(Box::into_raw(context) as *mut libc::c_void);

        assert_eq!(result, 1);
    }

    #[test]
    fn child_process_run_handles_empty_arguments_and_environment() {
        let context = Box::new(make_child(
            "/definitely/not/a/real/program",
            None,
            vec![],
            vec![],
        ));

        let result =
            ChildProcess::run(Box::into_raw(context) as *mut libc::c_void);

        assert_eq!(result, 1);
    }
}
