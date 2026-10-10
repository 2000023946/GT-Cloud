#[cfg(all(test, target_os = "linux"))]
mod tests {
    use crate::infrastructure::runtime::{
        states::process_state::ProcessState,
        start_runtime::helpers::create_child_process::create_child_process,
    };

    fn make_state() -> ProcessState {
        ProcessState {
            program: "/bin/echo".into(),
            args: vec!["hello".into()],
            working_directory: Some("/tmp".into()),
            environment: vec![("MODE".into(), "test".into())],
            pid: None,
        }
    }

    #[test]
    fn copies_all_fields() {
        let child = create_child_process(make_state());

        assert_eq!(child.program, "/bin/echo");
        assert_eq!(child.args, vec!["hello"]);
        assert_eq!(child.working_directory.as_deref(), Some("/tmp"));
        assert_eq!(child.environment, vec![("MODE".into(), "test".into())]);
    }

    #[test]
    fn preserves_empty_fields() {
        let state = ProcessState {
            program: "/bin/true".into(),
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
    fn preserves_multiple_arguments_and_environment() {
        let state = ProcessState {
            program: "/bin/echo".into(),
            args: vec!["one".into(), "two words".into()],
            working_directory: Some("/".into()),
            environment: vec![
                ("FIRST".into(), "1".into()),
                ("SECOND".into(), "two".into()),
            ],
            pid: Some(1234),
        };

        let child = create_child_process(state);

        assert_eq!(child.args, vec!["one", "two words"]);
        assert_eq!(child.working_directory.as_deref(), Some("/"));
        assert_eq!(
            child.environment,
            vec![("FIRST".into(), "1".into()), ("SECOND".into(), "two".into())]
        );
    }
}